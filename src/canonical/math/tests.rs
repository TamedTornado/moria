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
const ZERO_ITERATIONS: [u64; 32] = [
    0x55f2b6460e784a11,
    0x59347f5ef38f1dd3,
    0x776339ecdd922386,
    0x09606b85867d311d,
    0x4cb9676c5915fe50,
    0xac5d111110cfd4ad,
    0xd7213c09a80a62e1,
    0xd77fbfacaf5bee36,
    0x3591098a570a44f3,
    0x2c0004ac84ecbfe8,
    0xdceb45a7f5bfd4d5,
    0xa2f5d0a041785511,
    0x8750f6b950df5861,
    0x712bed2cf0230815,
    0x96d212d5f5a480e6,
    0x258162724fafc430,
    0x988423a9c73c99b4,
    0xa70a2324828e5dbc,
    0x3ffcc0223c54b047,
    0x6951e7ccadca3c87,
    0x6cb336f7fef43af3,
    0xbaa0c7fc0d02099d,
    0x3d437049b61de799,
    0x6d1ef03f8b8c1e0b,
    0xed31d54a6422bee3,
    0xbfd7565f36b99f98,
    0xed528d613c367259,
    0x0e442e9b0405ad10,
    0xd9b404859e79fbc6,
    0x282176b57b6bed77,
    0x49a97fe229eebc12,
    0xcb0772401b545ba3,
];

const ABOVE_CENTER_ITERATIONS: [u64; 32] = [
    0x81ad00e562556f51,
    0x8afdc5d32f6f7093,
    0xf9976b1ffadac646,
    0x8d81c73865bd1e5d,
    0x22013209377ed390,
    0x03a31514581ce358,
    0x00300ad416c34421,
    0x9c97d368416f1931,
    0xca58819c0423ec2e,
    0xfcf63a05d0c042a8,
    0xaccae5cc6cbcf215,
    0xcce71b46f150ee51,
    0x5b982c19fd04bfa1,
    0x4121c020044bd455,
    0x1651e1a2d9cf25a6,
    0x8872d620f8cbf7e1,
    0xc43cee491b173274,
    0xd393f1e796bbac7c,
    0x19982002ad563e07,
    0x3aef215874cdec47,
    0x3a2c9f2e9fe8a1b3,
    0xf9676be0223d22ba,
    0x68fc3ae909f88059,
    0x416625a037b1854b,
    0x906c310f5d115370,
    0x6b2cb20e27474669,
    0x1cb5e911a7763099,
    0x39fcf93a57e045d0,
    0x056ccf24f2549486,
    0xf0f1b89ed2bb26b7,
    0xdd6f39ca4bafc576,
    0x56d4ab97af104965,
];

const BELOW_MIDPOINT_ITERATIONS: [u64; 32] = [
    0x3eae4460eebb1525,
    0x3025e89484d69bbb,
    0x53d0a53f112d51de,
    0x0300b3eb556712f5,
    0x9ec3472f86d1e710,
    0x79f5d5bd0a08c433,
    0x7dd4937115f23612,
    0xe60be2af7d01fe1b,
    0xc231519af86b985f,
    0x7213f6ac08ffdf2e,
    0x392dcaaa87947eba,
    0xe207f59962a93a47,
    0xf6ac0072a2544ec9,
    0xa1d8236e537aad91,
    0x50aa68278b34a854,
    0xa00b779edb28a9dd,
    0x81bf2811890a3a68,
    0x4d109e3b1c158842,
    0xea48a1f2f5c734de,
    0x116b1e4bb0ff8c30,
    0x7e14d430873495da,
    0xa08dd2ddd9669bc7,
    0xeb38d653a4cae3e7,
    0x8ef4a146981d952d,
    0x7e092fbc6242303d,
    0x37238b8a33f9f666,
    0x60339ac30a2332c7,
    0xe542232a9a94abcd,
    0x1306e1c80ffafc60,
    0x97fa912658806301,
    0x4989d06706891941,
    0x6e74fe7ddf9b1f0b,
];

const MIDPOINT_ITERATIONS: [u64; 32] = [
    0xcfd2dcc4242098d8,
    0x7d73f261b9b4ad5a,
    0x158394906385454b,
    0x3ad37470ec468135,
    0xc1afb9dcf0217fe3,
    0x34e71c84f86056f8,
    0x840dee5142f80cda,
    0xc867a632c052e026,
    0x0e4266569a281b89,
    0x95eec9788a2ecbbb,
    0x8dc86fc08f8f3dca,
    0x37712fb5bc38e905,
    0x2b3e7293ead7f28a,
    0x54175763260390a0,
    0x34cdbe74862443d2,
    0xfb5ba87750dc9632,
    0x50dcfd197250fdb7,
    0x69e9c8e3031603e7,
    0x9b14b53195dff04f,
    0xc0cd32af72008a5c,
    0xe0ea780314958a19,
    0x852a20042e79bd78,
    0xdd738b124db2082b,
    0x6d54f1e1eb8131b2,
    0x13934260d192391a,
    0x33ef41727c44b413,
    0x2e27b4f2f0f5224b,
    0x1add1c0c7c510fbc,
    0xe6a40db7262c221c,
    0x328bb6a3d98b2ee0,
    0x7d87d2dfc95330e9,
    0x92320213ec9922aa,
];

const ABOVE_MIDPOINT_ITERATIONS: [u64; 32] = [
    0xa41a1224d0460018,
    0xa82c27c4db4bd81a,
    0xe62038dff845870b,
    0x63e2c33b5b003bf5,
    0xf45b7c2d6d218aa3,
    0x5f9f51e819f781b8,
    0x585523b1ef1d741a,
    0x0d2d56da3bd85362,
    0x90761789b76fe4c9,
    0xbf07b1aa477bd17b,
    0xb5e08fe020001e0a,
    0xbef90b689e5cd8c5,
    0x0eecfbff2438d1f1,
    0xd497889641d8ebe0,
    0x60868913d9fedc92,
    0xd4f70857c1de23f2,
    0x0650bb51f55086c4,
    0x92f897ad71cee527,
    0x697227ae2511fd20,
    0xf5453495cd960e83,
    0xc1703003d4cde048,
    0x08547fe22a0987b8,
    0x5cf359df31dcaceb,
    0x419c274297a698f2,
    0x87d523c9d3ed5ceb,
    0x60791035907202d3,
    0xb15214d0ec84ec8b,
    0x4695e6abd02ba87c,
    0x69ce6d9521bbec5c,
    0x5e4481432d65c7a0,
    0x252cbf2bcb7d524d,
    0x5ea1aa0c0531fc0c,
];

const MAXIMUM_TURN_ITERATIONS: [u64; 32] = [
    0xf944ec20f894940c,
    0xf4e1a2e15151f032,
    0xcdf17643aa887877,
    0x4c95933ee5cff431,
    0xa76ac6b730b95e7a,
    0xbc7078436675cb43,
    0x781f5db4ace925a1,
    0xa757c399fe054f64,
    0xf564ae7d7af39de8,
    0x971c570ad7bd3abb,
    0xe5dd4080a85bcde9,
    0x7e373ab326ef726e,
    0xaffff272eaceb0c3,
    0x97cfaad2ce57d1da,
    0x1ec0fc8270efd7ea,
    0x042082c308f6133a,
    0x875bfeb8d2ba16b6,
    0xa0f1a353770e0bdd,
    0x681f8ec529bb6589,
    0xfcd77b84ef8699bc,
    0x4c35fb39e483b73d,
    0x87163793cfa7eb67,
    0x17c2fb5cf14f009d,
    0x184a5db6d0533cda,
    0xbfaa778c130dbd70,
    0x4660180da2e05bf8,
    0xb2186934d0430eff,
    0x0c62c146d423068f,
    0xbf680beb2fdb6bec,
    0x218bd3cc1b25b118,
    0xbf73b217e4273783,
    0x149042b3056df3d0,
];

fn iteration_fingerprint(words: [i64; 3]) -> u64 {
    words
        .into_iter()
        .flat_map(i64::to_le_bytes)
        .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

fn retained_iteration_fingerprints(angle: u32) -> &'static [u64; 32] {
    match angle & 0x3fff_ffff {
        0 => &ZERO_ITERATIONS,
        1 => &ABOVE_CENTER_ITERATIONS,
        0x1fff_ffff => &BELOW_MIDPOINT_ITERATIONS,
        0x2000_0000 => &MIDPOINT_ITERATIONS,
        0x2000_0001 => &ABOVE_MIDPOINT_ITERATIONS,
        0x3fff_ffff => &MAXIMUM_TURN_ITERATIONS,
        remainder => panic!("no retained CORDIC iteration ledger for {remainder:#010x}"),
    }
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

    let centers: [u32; 4] = [0x0000_0000, 0x4000_0000, 0x8000_0000, 0xc000_0000];
    let midpoints: [u32; 4] = [0x2000_0000, 0x6000_0000, 0xa000_0000, 0xe000_0000];
    let mut angles = Vec::new();
    for center in centers {
        angles.extend([center.wrapping_sub(1), center, center.wrapping_add(1)]);
    }
    for midpoint in midpoints {
        angles.extend([midpoint - 1, midpoint, midpoint + 1]);
    }
    angles.push(u32::MAX);

    for angle in angles {
        let actual = cordic_iterations(angle).unwrap();
        for (iteration, (state, expected)) in actual
            .into_iter()
            .zip(retained_iteration_fingerprints(angle))
            .enumerate()
        {
            assert_eq!(
                iteration_fingerprint(state.words()),
                *expected,
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
    let centers: [u32; 4] = [0x0000_0000, 0x4000_0000, 0x8000_0000, 0xc000_0000];
    let midpoints: [u32; 4] = [0x2000_0000, 0x6000_0000, 0xa000_0000, 0xe000_0000];
    let axes = [
        [0, 0, 0],
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, 0, -1],
        [1, 1, 1],
        [1, 1, 2],
        [i32::MIN, 0, 0],
        [i32::MAX, i32::MIN, i32::MAX],
        [i32::MAX, i32::MAX, i32::MAX],
        [2_054_521_149, -1_911_829_703, -38_241_389],
    ];
    let mut angles = Vec::new();
    for center in centers {
        angles.extend([center.wrapping_sub(1), center, center.wrapping_add(1)]);
    }
    for midpoint in midpoints {
        angles.extend([midpoint - 1, midpoint, midpoint + 1]);
    }
    angles.push(u32::MAX);
    angles
        .into_iter()
        .flat_map(|angle| {
            axes.into_iter()
                .map(move |axis| CordicParityCase { angle, axis })
        })
        .collect()
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
        for (iteration, expected) in retained_iteration_fingerprints(case.angle)
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
        let final_offset = base + 192 * 4;
        let actual_final = [
            i32::from_le_bytes(actual[final_offset..final_offset + 4].try_into().unwrap()),
            i32::from_le_bytes(
                actual[final_offset + 4..final_offset + 8]
                    .try_into()
                    .unwrap(),
            ),
        ];
        let expected_final = super::cordic::sine_cosine_q30(case.angle)
            .map(|(sine, cosine)| [sine, cosine])
            .expect("all u32 turn words are valid CORDIC inputs");
        assert_eq!(
            actual_final, expected_final,
            "GPU CORDIC final result for {case:?}"
        );

        let axis_offset = final_offset + 8;
        let actual_axis = [
            i32::from_le_bytes(actual[axis_offset..axis_offset + 4].try_into().unwrap()),
            i32::from_le_bytes(actual[axis_offset + 4..axis_offset + 8].try_into().unwrap()),
            i32::from_le_bytes(
                actual[axis_offset + 8..axis_offset + 12]
                    .try_into()
                    .unwrap(),
            ),
        ];
        let actual_failure = u32::from_le_bytes(
            actual[axis_offset + 12..axis_offset + 16]
                .try_into()
                .unwrap(),
        );
        let expected_axis = super::cordic::normalize_axis_q30(case.axis);
        match expected_axis {
            Ok(expected_axis) => {
                assert_eq!(
                    actual_axis, expected_axis,
                    "GPU normalized axis for {case:?}"
                );
                assert_eq!(actual_failure, 0, "GPU axis success tag for {case:?}");
            }
            Err(expected_failure) => {
                assert_eq!(actual_axis, [0; 3], "GPU failed axis payload for {case:?}");
                assert_eq!(
                    actual_failure,
                    u32::from(expected_failure.wire_tag()),
                    "GPU axis failure tag for {case:?}"
                );
            }
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
