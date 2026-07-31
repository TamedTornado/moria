use super::{fixed::FixedI32, wide::WideI64};
use crate::facade::CanonicalFailure;
use std::{
    future::Future,
    pin::pin,
    sync::mpsc,
    task::{Context, Poll, Waker},
    time::Duration,
};

// These FNV-1a fingerprints are retained from an independently generated
// TECH-071 ledger.  Each fingerprint covers the little-endian `(x, y, z)`
// words of one iteration, so changing a recurrence step cannot be hidden by
// a matching CPU/WGSL implementation change.
#[allow(dead_code)]
const ZERO_ITERATIONS: [u64; 32] = [
    0x55f2b6460e784a11,
    0x4bb283928fd33357,
    0xce473ced0ee33e31,
    0x1ab480e004ea4fca,
    0xd59d8eb500cc7b7f,
    0xdc6b4ff41feba112,
    0x6662fe89b5858f60,
    0x7062c3adc8e6237b,
    0x7328b6a4e648e443,
    0xc9f05511fa8cefe4,
    0x66a11556c3ccdaba,
    0xb5a9e5ca833a8dcd,
    0xde77cfd7fc83157c,
    0xa5b00cbfb928099b,
    0x28d980e9383b8be4,
    0xdfc4dd5d241205cc,
    0xc12f9f04a2134683,
    0x564751b8dac7c1ee,
    0x391f309757b4d2f2,
    0xfe2825fc6e583944,
    0x731fccaedaed5913,
    0x7bbda025271057c0,
    0x80c4be8f6990f91c,
    0x8f39f24d1450ad82,
    0xbb2446a00dc7cb37,
    0x8642a5050b076a1a,
    0xcf38ff704c0387a1,
    0xea0430d01d1a8927,
    0xc6843d1357555d20,
    0xd5488d669cdb5b85,
    0x9b3cbbd90386db44,
    0xf199b05df0946530,
];
#[allow(dead_code)]
const BELOW_MIDPOINT_ITERATIONS: [u64; 32] = [
    0x3eae4460eebb1525,
    0x1ff920f33bf7984f,
    0xdc190f8f4b38b8a4,
    0x1429e1861cf1dad6,
    0xbc8f45d2a65e2077,
    0x70f00699f2abed79,
    0x459946019e535186,
    0xefcbdfc7d206b8b9,
    0x757dcb2c191709e1,
    0x25a5a79a9f1363dc,
    0x5dbb87d06210f887,
    0xeeba142221774cd4,
    0xd10e47384b83d647,
    0x9528d3bd3cc87188,
    0xd6e0a5db0a7223a6,
    0x3c6daabffef55a41,
    0xa25b95a66dd4fa56,
    0x766f4e13c12827dd,
    0xc55a17d184e12914,
    0x27f619f10892d640,
    0xe1e1920c55ffd173,
    0x44157544cc62d2af,
    0x91dc1699da5256c5,
    0xc7cf8d5bc8c04868,
    0xb1b3f0532855f3e2,
    0xe4de37d2f713388d,
    0xaec0098f7b9802b9,
    0x493e41a1cdf65d2a,
    0x912019baed8ec13a,
    0x01864936f501e01b,
    0x52222d7580fd7105,
    0x8aaaf367a676287b,
];
#[allow(dead_code)]
const MIDPOINT_ITERATIONS: [u64; 32] = [
    0xcfd2dcc4242098d8,
    0x97c945a7b85bbb36,
    0xf86444ffa452d0c6,
    0xbd946db7bf12f68a,
    0xcd60baeb1b149638,
    0xbe6426ede57b17ea,
    0xd965b2120221d9c7,
    0xb60229c5934fe5cc,
    0x9f256b114303606f,
    0x06a5a5bd82bd0793,
    0x29ca2e714f5cc151,
    0xe20bae3ed019bdc4,
    0x6420cca9fc058dc2,
    0xd6b84f211592768f,
    0x2cba507752851923,
    0x1ced1f8dcbe86c47,
    0xdb489ea12bafd7a8,
    0x1dbcc9f00f541a7a,
    0x0406c21750576530,
    0x1e8aa356a3256b98,
    0x28600d568943bc6f,
    0xe6ba546c6c618ce2,
    0x0b69e7a2a03a4b15,
    0xc5b3c423282345aa,
    0x3450520d098fe921,
    0x519ad6f82e301bdc,
    0x9ed71684c688896d,
    0xb46028317c0ba85b,
    0x7a93ccef8b72cbc7,
    0x748e183df9d178c3,
    0x7ffc882d7d115b18,
    0xc90b38b0dd903ae3,
];
#[allow(dead_code)]
const ABOVE_MIDPOINT_ITERATIONS: [u64; 32] = [
    0xa41a1224d0460018,
    0x6b3f76e4a42e6c76,
    0x035bf4c71c7da735,
    0xe5ad0dd74f84b04a,
    0x504ff049561cf778,
    0x35dbe8d4cbac71aa,
    0x9e674a562e3ac616,
    0x33ce789276081c8c,
    0x26b72d557922bd2f,
    0xdaecdb1e2ee26ed3,
    0x5d80ec8f529c8091,
    0x0cc3e3a1f1b0e884,
    0x33563253e8456f91,
    0xaaffb757bc50954f,
    0x4a42c7f2afab8470,
    0xebe05d44adca6487,
    0x0b54cbae178a7168,
    0xecb007a6f13612ba,
    0x2c468cbdfec0f670,
    0xf910db1608e26437,
    0x002242afdadd912f,
    0x70e2128ce5130fa2,
    0x8e9447809bca1555,
    0x39f5a58c2a7e697b,
    0x689cdb655f7f93d0,
    0x25e20c58da55831c,
    0x731e4be572adf0ad,
    0xb0329c92c5d3663f,
    0xa352442a9a05d01a,
    0xc8beec1f0fcf0b84,
    0x7c82dffe3585b77d,
    0x628759ac6829044e,
];
#[allow(dead_code)]
const MAXIMUM_TURN_ITERATIONS: [u64; 32] = [
    0xf944ec20f894940c,
    0x6fa6714adebd4e3e,
    0x2a32b6754a4cba68,
    0x25aeadad46e5c8f6,
    0x7479324a84f16939,
    0x07cda74c63441cae,
    0xe345626411e601a8,
    0xbe589bc585321d66,
    0x9675ff3db2d4f784,
    0x02f0cc49283c004d,
    0x48493115118acea3,
    0x2da42b69a946451c,
    0x2d9822258d52bfc4,
    0xe51bdb5b895bb670,
    0x7580d5c4fff69623,
    0x5f07924e8faa58d3,
    0x1b15bb1d84198101,
    0x8f2dba36cfb200f5,
    0x2f88381473607712,
    0x41c8ac278a468707,
    0x8e56eaab86a53d19,
    0xa7a3f34301077432,
    0xe3dbaa606eb03b7c,
    0xa81a8a2ae128c440,
    0x61bda2eaf1c859ef,
    0x9d1db237309569b3,
    0xcbf80bbebb26dc8b,
    0x5d811ffa842151d8,
    0x8918c42f8347d322,
    0x420ee905429c256c,
    0x14d5b0ca35bedb2a,
    0x0cab26371bc1eeb9,
];

fn iteration_fingerprint(words: [i64; 3]) -> u64 {
    words
        .into_iter()
        .flat_map(i64::to_le_bytes)
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

fn expected_iteration_fingerprints(angle: u32) -> [u64; 32] {
    // Independent literal regeneration ledger: it intentionally does not
    // import the checked-in Rust table or CORDIC implementation.
    const ATAN: [i64; 32] = [
        576460752303423488,
        340304653033718298,
        179807632645220259,
        91273161881380487,
        45813697873323707,
        22929182573009054,
        11467389120678282,
        5734044481687724,
        2867065987018958,
        1433538461969102,
        716769914547871,
        358385042719534,
        179192532040472,
        89596267355325,
        44798133844548,
        22399066943135,
        11199533474175,
        5599766737413,
        2799883368747,
        1399941684379,
        699970842190,
        349985421095,
        174992710548,
        87496355274,
        43748177637,
        21874088818,
        10937044409,
        5468522205,
        2734261102,
        1367130551,
        683565276,
        341782638,
    ];
    let quadrant = (u64::from(angle) + 0x2000_0000) >> 30;
    let mut residual = (i64::from(angle) - i64::try_from(quadrant << 30).unwrap()) << 30;
    let mut x = 1_400_229_935_014_726_477_i64;
    let mut y = 0_i64;
    let mut goldens = [0_u64; 32];
    for (index, atan) in ATAN.into_iter().enumerate() {
        let (before_x, before_y, before_z) = (x, y, residual);
        if before_z >= 0 {
            x = before_x - (before_y >> index);
            y = before_y + (before_x >> index);
            residual = before_z - atan;
        } else {
            x = before_x + (before_y >> index);
            y = before_y - (before_x >> index);
            residual = before_z + atan;
        }
        goldens[index] = iteration_fingerprint([x, y, residual]);
    }
    goldens
}

#[test]
fn cordic_retains_center_midpoint_adjacent_and_maximum_turn_goldens() {
    use super::cordic::sine_cosine_q30;

    let cases = [
        (0x0000_0000, (0, 1_073_741_824)),
        (0x0000_0001, (1, 1_073_741_824)),
        (0x1fff_ffff, (759_250_124, 759_250_126)),
        (0x2000_0000, (759_250_125, 759_250_125)),
        (0x2000_0001, (759_250_126, 759_250_124)),
        (0x4000_0000, (1_073_741_824, 0)),
        (0x7fff_ffff, (1, -1_073_741_824)),
        (0x8000_0000, (0, -1_073_741_824)),
        (0xa000_0000, (-759_250_125, -759_250_125)),
        (0xc000_0000, (-1_073_741_824, 0)),
        (0xe000_0000, (-759_250_125, 759_250_125)),
        (0xffff_ffff, (-1, 1_073_741_824)),
    ];

    for (angle, expected) in cases {
        assert_eq!(sine_cosine_q30(angle).unwrap(), expected, "{angle:#010x}");
    }
}

#[test]
fn cordic_retains_every_iteration_at_all_required_turn_boundaries() {
    use super::cordic::cordic_iterations;

    let centers = [0x0000_0000, 0x4000_0000, 0x8000_0000, 0xc000_0000];
    let midpoints = [0x2000_0000, 0x6000_0000, 0xa000_0000, 0xe000_0000];
    let mut angles = Vec::from(centers);
    for midpoint in midpoints {
        angles.extend([midpoint - 1, midpoint, midpoint + 1]);
    }
    angles.push(u32::MAX);

    for angle in angles {
        let actual = cordic_iterations(angle).unwrap();
        for (iteration, (state, expected)) in actual
            .into_iter()
            .zip(expected_iteration_fingerprints(angle))
            .enumerate()
        {
            assert_eq!(
                iteration_fingerprint(state.words()),
                expected,
                "independent CORDIC iteration golden {iteration} for {angle:#010x}"
            );
        }
    }
}

#[test]
fn axis_normalization_retains_exact_q1_30_and_axis_failures() {
    use super::cordic::normalize_axis_q30;

    assert_eq!(normalize_axis_q30([1, 0, 0]), Ok([1_073_741_824, 0, 0]));
    assert_eq!(normalize_axis_q30([-1, 0, 0]), Ok([-1_073_741_824, 0, 0]));
    assert_eq!(normalize_axis_q30([0, 1, 0]), Ok([0, 1_073_741_824, 0]));
    assert_eq!(normalize_axis_q30([0, 0, -1]), Ok([0, 0, -1_073_741_824]));
    assert_eq!(
        normalize_axis_q30([1, 1, 1]),
        Ok([619_925_131, 619_925_131, 619_925_131])
    );
    // This exact squared-comparison tie selects the even candidate.
    assert_eq!(
        normalize_axis_q30([1, 1, 2]),
        Ok([438_353_264, 438_353_264, 876_706_528])
    );
    assert_eq!(
        normalize_axis_q30([i32::MIN, 0, 0]),
        Ok([-1_073_741_824, 0, 0])
    );
    assert_eq!(
        normalize_axis_q30([i32::MAX, i32::MIN, i32::MAX]),
        Ok([619_925_131, -619_925_131, 619_925_131])
    );
    assert_eq!(
        normalize_axis_q30([i32::MAX, i32::MAX, i32::MAX]),
        Ok([619_925_131, 619_925_131, 619_925_131])
    );
    assert_eq!(
        normalize_axis_q30([0, 0, 0]),
        Err(CanonicalFailure::ZeroAxis)
    );
}

const FIXED_PARITY_WGSL: &str = include_str!("../../../assets/shaders/canonical/math/fixed.wgsl");
const CORDIC_WGSL: &str = include_str!("../../../assets/shaders/canonical/math/cordic.wgsl");

#[test]
fn cordic_wgsl_parses_and_validates() {
    let module = naga::front::wgsl::parse_str(CORDIC_WGSL).expect("CORDIC WGSL must parse");
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .expect("CORDIC WGSL must validate");
}

#[test]
fn cordic_checked_in_table_matches_independent_ledger_and_wgsl() {
    use super::tables::{CORDIC_ATAN_TURNS_Q62, CORDIC_GAIN_INVERSE_Q61};

    let ledger_fingerprint = std::iter::once(CORDIC_GAIN_INVERSE_Q61)
        .chain(CORDIC_ATAN_TURNS_Q62)
        .flat_map(i64::to_le_bytes)
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        });
    assert_eq!(ledger_fingerprint, 0xc255_e76e_3d76_1e52);

    for value in std::iter::once(CORDIC_GAIN_INVERSE_Q61).chain(CORDIC_ATAN_TURNS_Q62) {
        let bits = value as u64;
        let wire = format!(
            "CordicWide(0x{:08x}u, 0x{:08x}u)",
            bits as u32,
            (bits >> 32) as u32
        );
        assert!(
            CORDIC_WGSL.contains(&wire),
            "checked-in WGSL CORDIC table is missing {wire}"
        );
    }
}

const CORDIC_PARITY_WORDS: usize = 198;

#[derive(Clone, Copy, Debug)]
struct CordicParityCase {
    angle: u32,
    axis: [i32; 3],
}

impl CordicParityCase {
    fn to_le_bytes(self) -> [u8; 16] {
        let mut bytes = [0_u8; 16];
        bytes[0..4].copy_from_slice(&self.angle.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.axis[0].to_le_bytes());
        bytes[8..12].copy_from_slice(&self.axis[1].to_le_bytes());
        bytes[12..16].copy_from_slice(&self.axis[2].to_le_bytes());
        bytes
    }
}

fn cordic_parity_cases() -> Vec<CordicParityCase> {
    let centers = [0x0000_0000, 0x4000_0000, 0x8000_0000, 0xc000_0000];
    let midpoints = [0x2000_0000, 0x6000_0000, 0xa000_0000, 0xe000_0000];
    let axes = [
        [1, 0, 0],
        [0, 1, 0],
        [i32::MIN, 0, 0],
        [i32::MAX, i32::MIN, i32::MAX],
        [0, 0, 0],
    ];
    let mut angles = Vec::from(centers);
    for midpoint in midpoints {
        angles.extend([midpoint - 1, midpoint, midpoint + 1]);
    }
    angles.push(u32::MAX);
    let mut cases: Vec<_> = angles
        .into_iter()
        .enumerate()
        .map(|(index, angle)| CordicParityCase {
            angle,
            axis: axes[index % axes.len()],
        })
        .collect();
    cases.push(CordicParityCase {
        angle: 0,
        axis: [2_054_521_149, -1_911_829_703, -38_241_389],
    });
    cases
}

fn run_cordic_gpu_parity(device: &wgpu::Device, queue: &wgpu::Queue, cases: &[CordicParityCase]) {
    let input_bytes: Vec<_> = cases.iter().flat_map(|case| case.to_le_bytes()).collect();
    let output_bytes = cases.len() * CORDIC_PARITY_WORDS * 4;
    let input = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("CORDIC parity input"),
        size: input_bytes.len() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let output = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("CORDIC parity output"),
        size: output_bytes as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("CORDIC parity readback"),
        size: output_bytes as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    queue.write_buffer(&input, 0, &input_bytes);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("canonical CORDIC parity"),
        source: wgpu::ShaderSource::Wgsl(CORDIC_WGSL.into()),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("CORDIC parity pipeline"),
        layout: None,
        module: &shader,
        entry_point: Some("cordic_parity"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("CORDIC parity bind group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output.as_entire_binding(),
            },
        ],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("CORDIC parity encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("CORDIC parity pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(cases.len() as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, output_bytes as u64);
    queue.submit([encoder.finish()]);
    let (mapped, receiver) = mpsc::sync_channel(1);
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            mapped
                .send(result)
                .expect("CORDIC map receiver remains live");
        });
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("CORDIC GPU polling succeeds");
    receiver
        .recv_timeout(Duration::from_secs(5))
        .expect("CORDIC readback completes within the bounded test wait")
        .expect("CORDIC readback maps successfully");
    let actual = readback.slice(..).get_mapped_range().to_vec();
    readback.unmap();
    for (case_index, case) in cases.iter().enumerate() {
        let base = case_index * CORDIC_PARITY_WORDS * 4;
        for (iteration, expected) in expected_iteration_fingerprints(case.angle)
            .iter()
            .enumerate()
        {
            let state = std::array::from_fn(|word| {
                let offset = base + (iteration * 3 + word) * 8;
                i64::from_le_bytes(actual[offset..offset + 8].try_into().unwrap())
            });
            assert_eq!(
                iteration_fingerprint(state),
                *expected,
                "independent CORDIC GPU golden {iteration} for {:#010x}",
                case.angle
            );
        }
    }
}

#[test]
fn cordic_wgsl_executes_every_iteration_and_axis_golden() {
    let instance = wgpu::Instance::default();
    let adapter = wait_for(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
        .expect("a GPU adapter is required for CORDIC parity coverage");
    let (device, queue) = wait_for(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("CORDIC CPU/WGSL parity"),
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::downlevel_defaults(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::MemoryUsage,
        trace: wgpu::Trace::Off,
    }))
    .expect("a GPU device is required for CORDIC parity coverage");
    let cases = cordic_parity_cases();
    run_cordic_gpu_parity(&device, &queue, &cases);
}

// Keep this test-only wire record beside the WGSL ABI. `left` and `right` are
// deliberately full signed words; sign-extending i32 inputs would not test the
// portable two-word helpers.
#[derive(Clone, Copy, Debug)]
struct FixedParityCase {
    left: i64,
    right: i64,
    shift: u32,
    operation: u32,
}

impl FixedParityCase {
    fn to_le_bytes(self) -> [u8; 24] {
        let mut bytes = [0_u8; 24];
        bytes[0..8].copy_from_slice(&self.left.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.right.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.shift.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.operation.to_le_bytes());
        bytes
    }
}

fn wait_for<T>(future: impl Future<Output = T>) -> T {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    for _ in 0..1_000 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    panic!("GPU initialization did not complete within the bounded test wait");
}

fn oracle_failure(error: CanonicalFailure) -> (i32, u32, u32) {
    (0, 0, u32::from(error.wire_tag()))
}

fn oracle_wire(value: i64) -> (i32, u32, u32) {
    (value as i32, (value as u64 >> 32) as u32, 0)
}

fn oracle_round(numerator: i128, denominator: i128) -> i128 {
    let quotient = numerator / denominator;
    let remainder = (numerator % denominator).unsigned_abs();
    let denominator_magnitude = denominator.unsigned_abs();
    let complement = denominator_magnitude - remainder;
    if remainder < complement || (remainder == complement && quotient & 1 == 0) {
        quotient
    } else if (numerator < 0) != (denominator < 0) {
        quotient - 1
    } else {
        quotient + 1
    }
}

fn oracle_sqrt(radicand: u64) -> u32 {
    let mut low = 0_u64;
    let mut high = 1_u64 << 32;
    while low + 1 < high {
        let middle = (low + high) / 2;
        if middle * middle <= radicand {
            low = middle;
        } else {
            high = middle;
        }
    }
    let lower_distance = radicand - low * low;
    let upper_distance = (low + 1) * (low + 1) - radicand;
    if lower_distance < upper_distance || (lower_distance == upper_distance && low & 1 == 0) {
        low as u32
    } else {
        (low + 1) as u32
    }
}

fn order_word(order: core::cmp::Ordering) -> i64 {
    match order {
        core::cmp::Ordering::Less => -1,
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
    }
}

/// Exact reference for the parity wire ABI. It intentionally does not call
/// production fixed or wide helpers, so agreement cannot be self-fulfilling.
fn exact_oracle(case: FixedParityCase, fractional_bits: u32) -> (i32, u32, u32) {
    if fractional_bits > 16 {
        return oracle_failure(CanonicalFailure::InvalidFixedFormat);
    }
    if case.operation <= 9
        && (i32::try_from(case.left).is_err() || i32::try_from(case.right).is_err())
    {
        return oracle_failure(CanonicalFailure::Nonrepresentable);
    }
    let left = case.left as i32;
    let right = case.right as i32;
    match case.operation {
        0 => i32::try_from(i64::from(left) + i64::from(right)).map_or_else(
            |_| oracle_failure(CanonicalFailure::ArithmeticOverflow),
            |value| oracle_wire(i64::from(value)),
        ),
        1 => i32::try_from(oracle_round(
            i128::from(left) * i128::from(right),
            1_i128 << fractional_bits,
        ))
        .map_or_else(
            |_| oracle_failure(CanonicalFailure::Nonrepresentable),
            |value| oracle_wire(i64::from(value)),
        ),
        2 if right == 0 => oracle_failure(CanonicalFailure::DivisionByZero),
        2 => i32::try_from(oracle_round(
            i128::from(left) << fractional_bits,
            i128::from(right),
        ))
        .map_or_else(
            |_| oracle_failure(CanonicalFailure::Nonrepresentable),
            |value| oracle_wire(i64::from(value)),
        ),
        3 if left < 0 => oracle_failure(CanonicalFailure::NegativeSquareRoot),
        3 => oracle_wire(i64::from(oracle_sqrt(
            (left as u32 as u64) << fractional_bits,
        ))),
        4 if case.shift > 31 => oracle_failure(CanonicalFailure::InvalidShift),
        4 => oracle_wire(oracle_round(i128::from(left), 1_i128 << case.shift) as i64),
        5 => i32::try_from(i64::from(left) - i64::from(right)).map_or_else(
            |_| oracle_failure(CanonicalFailure::ArithmeticOverflow),
            |value| oracle_wire(i64::from(value)),
        ),
        6 if left == i32::MIN => oracle_failure(CanonicalFailure::ArithmeticOverflow),
        6 => oracle_wire(i64::from(-left)),
        7 if left == i32::MIN => oracle_failure(CanonicalFailure::ArithmeticOverflow),
        7 => oracle_wire(i64::from(left.abs())),
        8 if right == 0 => oracle_failure(CanonicalFailure::DivisionByZero),
        8 if left == i32::MIN && right == -1 => {
            oracle_failure(CanonicalFailure::ArithmeticOverflow)
        }
        8 => {
            let quotient = left / right;
            let remainder = left % right;
            oracle_wire(i64::from(if remainder != 0 && (left < 0) != (right < 0) {
                quotient - 1
            } else {
                quotient
            }))
        }
        9 if case.shift > 31 => oracle_failure(CanonicalFailure::InvalidShift),
        9 => oracle_wire(i64::from(left >> case.shift)),
        10 => case.left.checked_add(case.right).map_or_else(
            || oracle_failure(CanonicalFailure::ArithmeticOverflow),
            oracle_wire,
        ),
        11 => case.left.checked_sub(case.right).map_or_else(
            || oracle_failure(CanonicalFailure::ArithmeticOverflow),
            oracle_wire,
        ),
        13 if case.shift > 63 => oracle_failure(CanonicalFailure::InvalidShift),
        13 => oracle_wire(case.left >> case.shift),
        14 if case.right == 0 => oracle_failure(CanonicalFailure::DivisionByZero),
        14 if case.left == i64::MIN && case.right == -1 => {
            oracle_failure(CanonicalFailure::ArithmeticOverflow)
        }
        14 => {
            let quotient = case.left / case.right;
            let remainder = case.left % case.right;
            oracle_wire(if remainder != 0 && (case.left < 0) != (case.right < 0) {
                quotient - 1
            } else {
                quotient
            })
        }
        15 => oracle_wire(order_word(case.left.cmp(&case.right))),
        16 => oracle_wire(order_word((case.left as u64).cmp(&(case.right as u64)))),
        17 => case.left.checked_neg().map_or_else(
            || oracle_failure(CanonicalFailure::ArithmeticOverflow),
            oracle_wire,
        ),
        _ => oracle_failure(CanonicalFailure::InvalidShift),
    }
}

fn production_failure(error: CanonicalFailure) -> (i32, u32, u32) {
    oracle_failure(error)
}

fn production_i32(result: Result<i32, CanonicalFailure>) -> (i32, u32, u32) {
    result.map_or_else(production_failure, |value| oracle_wire(i64::from(value)))
}

fn production_shift(shift: u32) -> Result<u8, CanonicalFailure> {
    u8::try_from(shift).map_err(|_| CanonicalFailure::InvalidShift)
}

fn production_fixed<const FRACTIONAL_BITS: u8>(case: FixedParityCase) -> (i32, u32, u32) {
    let left = match i32::try_from(case.left) {
        Ok(value) => value,
        Err(_) => return production_failure(CanonicalFailure::Nonrepresentable),
    };
    let right = match i32::try_from(case.right) {
        Ok(value) => value,
        Err(_) => return production_failure(CanonicalFailure::Nonrepresentable),
    };
    let left = match FixedI32::<FRACTIONAL_BITS>::try_from_raw(left) {
        Ok(value) => value,
        Err(error) => return production_failure(error),
    };
    let right = match FixedI32::<FRACTIONAL_BITS>::try_from_raw(right) {
        Ok(value) => value,
        Err(error) => return production_failure(error),
    };
    match case.operation {
        0 => production_i32(left.try_add(right).map(FixedI32::raw)),
        1 => production_i32(left.try_mul(right).map(FixedI32::raw)),
        2 => production_i32(left.try_div(right).map(FixedI32::raw)),
        3 => production_i32(left.try_sqrt().map(FixedI32::raw)),
        4 => production_i32(production_shift(case.shift).and_then(|shift| left.try_narrow(shift))),
        5 => production_i32(left.try_sub(right).map(FixedI32::raw)),
        6 => production_i32(left.try_neg().map(FixedI32::raw)),
        7 => production_i32(left.try_abs().map(FixedI32::raw)),
        8 => production_i32(super::fixed::floor_div(left.raw(), right.raw())),
        9 => production_i32(
            production_shift(case.shift)
                .and_then(|shift| super::fixed::floor_shift_right(left.raw(), shift)),
        ),
        _ => production_failure(CanonicalFailure::InvalidShift),
    }
}

fn production_wide(case: FixedParityCase) -> (i32, u32, u32) {
    let left = WideI64::from_i64(case.left);
    let right = WideI64::from_i64(case.right);
    let result = match case.operation {
        10 => left.checked_add(right).map(WideI64::to_i64),
        11 => left.checked_sub(right).map(WideI64::to_i64),
        13 => production_shift(case.shift)
            .and_then(|shift| left.floor_shift_right(shift))
            .map(WideI64::to_i64),
        14 => left.floor_div(right).map(WideI64::to_i64),
        15 => Ok(order_word(left.cmp_signed(right))),
        16 => Ok(order_word(
            left.to_i64()
                .cast_unsigned()
                .cmp(&right.to_i64().cast_unsigned()),
        )),
        17 => case
            .left
            .checked_neg()
            .ok_or(CanonicalFailure::ArithmeticOverflow),
        _ => Err(CanonicalFailure::InvalidShift),
    };
    result.map_or_else(production_failure, oracle_wire)
}

fn production_fixed_dispatch(case: FixedParityCase, fractional_bits: u32) -> (i32, u32, u32) {
    macro_rules! call {
        ($fractional_bits:literal) => {
            production_fixed::<$fractional_bits>(case)
        };
    }
    match fractional_bits {
        0 => call!(0),
        1 => call!(1),
        2 => call!(2),
        3 => call!(3),
        4 => call!(4),
        5 => call!(5),
        6 => call!(6),
        7 => call!(7),
        8 => call!(8),
        9 => call!(9),
        10 => call!(10),
        11 => call!(11),
        12 => call!(12),
        13 => call!(13),
        14 => call!(14),
        15 => call!(15),
        16 => call!(16),
        _ => match FixedI32::<17>::try_from_raw(0) {
            Err(error) => production_failure(error),
            Ok(_) => unreachable!("an out-of-contract fixed split must be rejected"),
        },
    }
}

fn run_cpu_parity(cases: &[FixedParityCase], fractional_bits: u32) {
    for (index, case) in cases.iter().copied().enumerate() {
        let expected = exact_oracle(case, fractional_bits);
        let actual = if case.operation <= 9 {
            production_fixed_dispatch(case, fractional_bits)
        } else {
            production_wide(case)
        };
        assert_eq!(
            actual, expected,
            "production Rust parity mismatch for split {fractional_bits}, case {index}: {case:?}"
        );
    }
}

fn generated_fixed_cases(fractional_bits: u32) -> Vec<FixedParityCase> {
    let edges = [
        i32::MIN,
        i32::MIN + 1,
        -65_537,
        -7,
        -3,
        -2,
        -1,
        0,
        1,
        2,
        3,
        7,
        65_537,
        i32::MAX - 1,
        i32::MAX,
    ];
    let mut cases = Vec::new();
    for (index, left) in edges.into_iter().enumerate() {
        let right = edges[(index * 7 + 3) % edges.len()];
        for operation in [0, 1, 2, 5, 8] {
            cases.push(FixedParityCase {
                left: i64::from(left),
                right: i64::from(right),
                shift: 0,
                operation,
            });
        }
        for operation in [3, 4, 6, 7, 9] {
            cases.push(FixedParityCase {
                left: i64::from(left),
                right: i64::from(right),
                shift: 31,
                operation,
            });
        }
    }
    for shift in [0, 1, 2, 15, 16, 30, 31, 32] {
        cases.push(FixedParityCase {
            left: -6,
            right: 0,
            shift,
            operation: 4,
        });
        cases.push(FixedParityCase {
            left: -9,
            right: 0,
            shift,
            operation: 9,
        });
    }
    if fractional_bits != 0 {
        let half = 1_i64 << (fractional_bits - 1);
        cases.extend([
            FixedParityCase {
                left: half,
                right: 3,
                shift: 0,
                operation: 1,
            },
            FixedParityCase {
                left: -half,
                right: 3,
                shift: 0,
                operation: 1,
            },
            FixedParityCase {
                left: 3,
                right: 2,
                shift: 0,
                operation: 2,
            },
            FixedParityCase {
                left: -3,
                right: 2,
                shift: 0,
                operation: 2,
            },
        ]);
    }
    cases.extend([
        FixedParityCase {
            left: i64::from(i32::MAX),
            right: 1,
            shift: 0,
            operation: 0,
        },
        FixedParityCase {
            left: i64::from(i32::MIN),
            right: 1,
            shift: 0,
            operation: 5,
        },
        FixedParityCase {
            left: i64::from(i32::MAX),
            right: i64::from(i32::MAX),
            shift: 0,
            operation: 1,
        },
        FixedParityCase {
            left: 0,
            right: 0,
            shift: 0,
            operation: 2,
        },
        FixedParityCase {
            left: -1,
            right: 0,
            shift: 0,
            operation: 3,
        },
        FixedParityCase {
            left: 2,
            right: 0,
            shift: 0,
            operation: 3,
        },
        FixedParityCase {
            left: i64::from(i32::MAX) + 1,
            right: 0,
            shift: 0,
            operation: 0,
        },
    ]);
    let divisor = 1_i64 << (fractional_bits + 1);
    cases.extend([
        FixedParityCase {
            left: 1,
            right: divisor,
            shift: 0,
            operation: 2,
        },
        FixedParityCase {
            left: -1,
            right: divisor,
            shift: 0,
            operation: 2,
        },
        FixedParityCase {
            left: 6,
            right: 0,
            shift: 2,
            operation: 4,
        },
        FixedParityCase {
            left: -6,
            right: 0,
            shift: 2,
            operation: 4,
        },
    ]);
    cases
}

fn generated_wide_cases() -> Vec<FixedParityCase> {
    let words = [
        i64::MIN,
        i64::MIN + 1,
        -9,
        -1,
        0,
        1,
        9,
        i64::MAX - 1,
        i64::MAX,
    ];
    let mut cases = Vec::new();
    for (index, left) in words.into_iter().enumerate() {
        let right = words[(index * 5 + 2) % words.len()];
        for operation in [10, 11, 14, 15, 16, 17] {
            cases.push(FixedParityCase {
                left,
                right,
                shift: 0,
                operation,
            });
        }
        for shift in [0, 1, 31, 32, 63, 64] {
            cases.push(FixedParityCase {
                left,
                right,
                shift,
                operation: 13,
            });
        }
    }
    cases.extend([
        FixedParityCase {
            left: i64::MAX,
            right: 1,
            shift: 0,
            operation: 10,
        },
        FixedParityCase {
            left: i64::MIN,
            right: 1,
            shift: 0,
            operation: 11,
        },
        FixedParityCase {
            left: i64::MIN,
            right: -1,
            shift: 0,
            operation: 14,
        },
        FixedParityCase {
            left: i64::MIN,
            right: 0,
            shift: 0,
            operation: 17,
        },
    ]);
    cases
}

fn run_gpu_parity(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    cases: &[FixedParityCase],
    fractional_bits: u32,
) {
    let input_bytes: Vec<_> = cases.iter().flat_map(|case| case.to_le_bytes()).collect();
    let expected_bytes: Vec<_> = cases
        .iter()
        .flat_map(|case| {
            let (value, high, failure) = exact_oracle(*case, fractional_bits);
            [
                value.to_le_bytes(),
                high.to_le_bytes(),
                failure.to_le_bytes(),
            ]
            .concat()
        })
        .collect();
    let input = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("fixed-point parity input"),
        size: input_bytes.len() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let output = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("fixed-point parity output"),
        size: expected_bytes.len() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("fixed-point parity readback"),
        size: expected_bytes.len() as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    queue.write_buffer(&input, 0, &input_bytes);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("canonical fixed-point parity"),
        source: wgpu::ShaderSource::Wgsl(FIXED_PARITY_WGSL.into()),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("fixed-point parity pipeline"),
        layout: None,
        module: &shader,
        entry_point: Some("fixed_parity"),
        compilation_options: wgpu::PipelineCompilationOptions {
            constants: &[("FRACTIONAL_BITS", f64::from(fractional_bits))],
            zero_initialize_workgroup_memory: true,
        },
        cache: None,
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("fixed-point parity bind group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output.as_entire_binding(),
            },
        ],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("fixed-point parity encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("fixed-point parity pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(cases.len() as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, expected_bytes.len() as u64);
    queue.submit([encoder.finish()]);
    let (mapped, receiver) = mpsc::sync_channel(1);
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            mapped
                .send(result)
                .expect("parity map receiver remains live");
        });
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("parity GPU polling succeeds");
    receiver
        .recv_timeout(Duration::from_secs(5))
        .expect("parity readback completes within the bounded test wait")
        .expect("parity readback maps successfully");
    let actual_bytes = readback.slice(..).get_mapped_range().to_vec();
    readback.unmap();
    for (index, (actual, expected)) in actual_bytes
        .chunks_exact(12)
        .zip(expected_bytes.chunks_exact(12))
        .enumerate()
    {
        assert_eq!(
            actual, expected,
            "GPU parity mismatch for split {fractional_bits}, case {index}: {:?}",
            cases[index]
        );
    }
}

#[test]
fn generated_cpu_wgsl_parity_covers_every_split_and_two_word_operation() {
    let instance = wgpu::Instance::default();
    let adapter = wait_for(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
        .expect("a GPU adapter is required for CPU/WGSL parity coverage");
    let (device, queue) = wait_for(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("fixed-point CPU/WGSL parity"),
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::downlevel_defaults(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::MemoryUsage,
        trace: wgpu::Trace::Off,
    }))
    .expect("a GPU device is required for CPU/WGSL parity coverage");
    for fractional_bits in 0..=16 {
        let cases = generated_fixed_cases(fractional_bits);
        run_cpu_parity(&cases, fractional_bits);
        run_gpu_parity(&device, &queue, &cases, fractional_bits);
    }
    let wide_cases = generated_wide_cases();
    run_cpu_parity(&wide_cases, 0);
    run_gpu_parity(&device, &queue, &wide_cases, 0);
    let invalid_format_cases = generated_fixed_cases(0);
    run_cpu_parity(&invalid_format_cases, 17);
    run_gpu_parity(&device, &queue, &invalid_format_cases, 17);
}

#[test]
fn wide_words_preserve_signed_boundaries() {
    assert_eq!(WideI64::from_i64(-9).to_i64(), -9);
    assert_eq!(
        WideI64::from_i64(-9).to_le_bytes(),
        [247, 255, 255, 255, 255, 255, 255, 255]
    );
    assert_eq!(
        WideI64::from_i64(-9).cmp_signed(WideI64::from_i64(0)),
        core::cmp::Ordering::Less
    );
    assert_eq!(
        WideI64::from_i64(i64::MAX).checked_add(WideI64::from_i64(1)),
        Err(CanonicalFailure::ArithmeticOverflow)
    );
    assert_eq!(
        WideI64::from_i64(i64::MIN).checked_sub(WideI64::from_i64(1)),
        Err(CanonicalFailure::ArithmeticOverflow)
    );
    assert_eq!(
        WideI64::from_i64(-9).floor_shift_right(3).unwrap().to_i64(),
        -2
    );
    assert_eq!(
        WideI64::from_i64(-3)
            .checked_mul(WideI64::from_i64(7))
            .unwrap()
            .to_i64(),
        -21
    );
    assert_eq!(
        WideI64::from_i64(1)
            .floor_div(WideI64::from_i64(-2))
            .unwrap()
            .to_i64(),
        -1
    );
}
