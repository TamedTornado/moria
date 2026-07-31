use moria::{
    canonical::math::{FixedI32, WideI64, floor_div, floor_shift_right},
    facade::CanonicalFailure,
};
use std::{
    future::Future,
    pin::pin,
    sync::mpsc,
    task::{Context, Poll, Waker},
    time::Duration,
};

const FIXED_PARITY_WGSL: &str = include_str!("../assets/shaders/canonical/math/fixed.wgsl");

#[derive(Clone, Copy)]
struct FixedParityCase {
    left: i32,
    right: i32,
    fractional_bits: u32,
    shift: u32,
    operation: u32,
}

impl FixedParityCase {
    fn to_le_bytes(self) -> [u8; 20] {
        let mut bytes = [0_u8; 20];
        bytes[0..4].copy_from_slice(&self.left.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.right.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.fractional_bits.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.shift.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.operation.to_le_bytes());
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

fn cpu_fixed_result(case: FixedParityCase) -> (i32, u32) {
    macro_rules! operation_for_split {
        ($fractional_bits:literal) => {{
            let left = FixedI32::<$fractional_bits>::try_from_raw(case.left).unwrap();
            let right = FixedI32::<$fractional_bits>::try_from_raw(case.right).unwrap();
            let result = match case.operation {
                0 => left.try_add(right).map(FixedI32::raw),
                1 => left.try_mul(right).map(FixedI32::raw),
                2 => left.try_div(right).map(FixedI32::raw),
                3 => left.try_sqrt().map(FixedI32::raw),
                4 => left.try_narrow(case.shift as u8),
                _ => unreachable!(),
            };
            match result {
                Ok(value) => (value, 0),
                Err(CanonicalFailure::ArithmeticOverflow) => (0, 1),
                Err(CanonicalFailure::DivisionByZero) => (0, 2),
                Err(CanonicalFailure::InvalidShift) => (0, 3),
                Err(CanonicalFailure::NegativeSquareRoot) => (0, 4),
                Err(CanonicalFailure::Nonrepresentable) => (0, 5),
                Err(error) => panic!("unexpected fixed-point failure: {error:?}"),
            }
        }};
    }
    match case.fractional_bits {
        0 => operation_for_split!(0),
        1 => operation_for_split!(1),
        2 => operation_for_split!(2),
        3 => operation_for_split!(3),
        4 => operation_for_split!(4),
        5 => operation_for_split!(5),
        6 => operation_for_split!(6),
        7 => operation_for_split!(7),
        8 => operation_for_split!(8),
        9 => operation_for_split!(9),
        10 => operation_for_split!(10),
        11 => operation_for_split!(11),
        12 => operation_for_split!(12),
        13 => operation_for_split!(13),
        14 => operation_for_split!(14),
        15 => operation_for_split!(15),
        16 => operation_for_split!(16),
        _ => unreachable!(),
    }
}

#[test]
fn arithmetic_rounds_half_ties_to_even_for_every_placement_split() {
    for fractional_bits in 0..=16 {
        match fractional_bits {
            0 => assert_eq!(
                FixedI32::<0>::try_from_raw(3)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(1).unwrap()),
                Ok(FixedI32::try_from_raw(3).unwrap())
            ),
            1 => assert_eq!(
                FixedI32::<1>::try_from_raw(1)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            2 => assert_eq!(
                FixedI32::<2>::try_from_raw(2)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            3 => assert_eq!(
                FixedI32::<3>::try_from_raw(4)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            4 => assert_eq!(
                FixedI32::<4>::try_from_raw(8)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            5 => assert_eq!(
                FixedI32::<5>::try_from_raw(16)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            6 => assert_eq!(
                FixedI32::<6>::try_from_raw(32)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            7 => assert_eq!(
                FixedI32::<7>::try_from_raw(64)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            8 => assert_eq!(
                FixedI32::<8>::try_from_raw(128)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            9 => assert_eq!(
                FixedI32::<9>::try_from_raw(256)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            10 => assert_eq!(
                FixedI32::<10>::try_from_raw(512)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            11 => assert_eq!(
                FixedI32::<11>::try_from_raw(1024)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            12 => assert_eq!(
                FixedI32::<12>::try_from_raw(2048)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            13 => assert_eq!(
                FixedI32::<13>::try_from_raw(4096)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            14 => assert_eq!(
                FixedI32::<14>::try_from_raw(8192)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            15 => assert_eq!(
                FixedI32::<15>::try_from_raw(16384)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            16 => assert_eq!(
                FixedI32::<16>::try_from_raw(32768)
                    .unwrap()
                    .try_mul(FixedI32::try_from_raw(3).unwrap()),
                Ok(FixedI32::try_from_raw(2).unwrap())
            ),
            _ => unreachable!(),
        }
    }
}

#[test]
fn division_and_square_root_match_exact_integer_oracles() {
    macro_rules! assert_split {
        ($fractional_bits:literal) => {{
            let one = FixedI32::<$fractional_bits>::try_from_raw(1 << $fractional_bits).unwrap();
            let two = FixedI32::<$fractional_bits>::try_from_raw(2 << $fractional_bits).unwrap();
            let four = FixedI32::<$fractional_bits>::try_from_raw(4 << $fractional_bits).unwrap();
            assert_eq!(
                one.try_div(two),
                Ok(FixedI32::try_from_raw(1 << ($fractional_bits - 1)).unwrap())
            );
            assert_eq!(
                four.try_sqrt(),
                Ok(FixedI32::try_from_raw(2 << $fractional_bits).unwrap())
            );
        }};
    }
    // `F=0` is deliberately separate because it has no fractional half tie.
    assert_eq!(
        FixedI32::<0>::try_from_raw(2).unwrap().try_sqrt(),
        Ok(FixedI32::try_from_raw(1).unwrap())
    );
    assert_split!(1);
    assert_split!(2);
    assert_split!(3);
    assert_split!(4);
    assert_split!(5);
    assert_split!(6);
    assert_split!(7);
    assert_split!(8);
    assert_split!(9);
    assert_split!(10);
    assert_split!(11);
    assert_split!(12);
    assert_split!(13);
    assert_split!(14);
    assert_split!(15);
    assert_split!(16);
}

#[test]
fn narrowing_uses_ties_to_even_for_negative_and_positive_values() {
    let positive_tie = FixedI32::<0>::try_from_raw(6).unwrap();
    let negative_tie = FixedI32::<0>::try_from_raw(-6).unwrap();
    assert_eq!(positive_tie.try_narrow(2), Ok(2));
    assert_eq!(negative_tie.try_narrow(2), Ok(-2));
    assert_eq!(FixedI32::<0>::try_from_raw(7).unwrap().try_narrow(2), Ok(2));
    assert_eq!(
        FixedI32::<0>::try_from_raw(-7).unwrap().try_narrow(2),
        Ok(-2)
    );
    assert_eq!(
        FixedI32::<0>::try_from_raw(1).unwrap().try_narrow(32),
        Err(CanonicalFailure::InvalidShift)
    );
}

#[test]
fn floor_division_covers_every_operand_sign_combination() {
    assert_eq!(floor_div(9, 8), Ok(1));
    assert_eq!(floor_div(-9, 8), Ok(-2));
    assert_eq!(floor_div(9, -8), Ok(-2));
    assert_eq!(floor_div(-9, -8), Ok(1));

    assert_eq!(
        WideI64::from_i64(9)
            .floor_div(WideI64::from_i64(8))
            .map(WideI64::to_i64),
        Ok(1)
    );
    assert_eq!(
        WideI64::from_i64(-9)
            .floor_div(WideI64::from_i64(8))
            .map(WideI64::to_i64),
        Ok(-2)
    );
    assert_eq!(
        WideI64::from_i64(9)
            .floor_div(WideI64::from_i64(-8))
            .map(WideI64::to_i64),
        Ok(-2)
    );
    assert_eq!(
        WideI64::from_i64(-9)
            .floor_div(WideI64::from_i64(-8))
            .map(WideI64::to_i64),
        Ok(1)
    );
}

#[test]
fn fixed_operations_report_stable_failures_and_floor_is_distinct() {
    assert_eq!(
        FixedI32::<17>::try_from_raw(0),
        Err(CanonicalFailure::InvalidFixedFormat)
    );
    let one = FixedI32::<1>::try_from_raw(1).unwrap();
    assert_eq!(
        one.try_div(FixedI32::try_from_raw(0).unwrap()),
        Err(CanonicalFailure::DivisionByZero)
    );
    assert_eq!(
        FixedI32::<1>::try_from_raw(-1).unwrap().try_sqrt(),
        Err(CanonicalFailure::NegativeSquareRoot)
    );
    assert_eq!(
        FixedI32::<1>::try_from_raw(i32::MAX).unwrap().try_add(one),
        Err(CanonicalFailure::ArithmeticOverflow)
    );
    assert_eq!(floor_div(-9, 8), Ok(-2));
    assert_eq!(floor_shift_right(-9, 3), Ok(-2));
    assert_eq!(floor_shift_right(-1, 31), Ok(-1));
    assert_eq!(
        floor_shift_right(1, 32),
        Err(CanonicalFailure::InvalidShift)
    );
}

#[test]
fn wide_words_match_signed_i64_arithmetic() {
    let value = WideI64::from_i64(-9);
    assert_eq!(value.to_i64(), -9);
    assert_eq!(
        value.to_le_bytes(),
        [247, 255, 255, 255, 255, 255, 255, 255]
    );
    assert_eq!(
        value.cmp_signed(WideI64::from_i64(0)),
        core::cmp::Ordering::Less
    );
    assert_eq!(value.floor_shift_right(3).unwrap().to_i64(), -2);
    assert_eq!(
        WideI64::from_i64(-3)
            .checked_mul(WideI64::from_i64(7))
            .unwrap()
            .to_i64(),
        -21
    );
    assert_eq!(
        WideI64::from_i64(i64::MAX).checked_add(WideI64::from_i64(1)),
        Err(CanonicalFailure::ArithmeticOverflow)
    );
}

#[test]
fn fixed_point_cpu_and_wgsl_compute_outputs_are_byte_identical() {
    let mut cases = vec![
        FixedParityCase {
            left: 7,
            right: -3,
            fractional_bits: 0,
            shift: 0,
            operation: 0,
        },
        FixedParityCase {
            left: -7,
            right: 0,
            fractional_bits: 0,
            shift: 2,
            operation: 4,
        },
        FixedParityCase {
            left: i32::MAX,
            right: 1,
            fractional_bits: 0,
            shift: 0,
            operation: 0,
        },
        FixedParityCase {
            left: 1,
            right: 0,
            fractional_bits: 0,
            shift: 0,
            operation: 2,
        },
        FixedParityCase {
            left: -1,
            right: 0,
            fractional_bits: 0,
            shift: 0,
            operation: 3,
        },
        FixedParityCase {
            left: 1,
            right: 0,
            fractional_bits: 0,
            shift: 32,
            operation: 4,
        },
    ];
    for fractional_bits in 0..=16_u32 {
        let unit = 1_i32 << fractional_bits;
        cases.extend([
            FixedParityCase {
                left: unit,
                right: 3 * unit,
                fractional_bits,
                shift: 0,
                operation: 1,
            },
            FixedParityCase {
                left: 3 * unit,
                right: 2 * unit,
                fractional_bits,
                shift: 0,
                operation: 2,
            },
            FixedParityCase {
                left: 4 * unit,
                right: 0,
                fractional_bits,
                shift: 0,
                operation: 3,
            },
        ]);
    }
    let mut input_bytes = Vec::with_capacity(cases.len() * 20);
    let mut expected_bytes = Vec::with_capacity(cases.len() * 8);
    for case in cases.iter().copied() {
        input_bytes.extend(case.to_le_bytes());
        let (value, failure) = cpu_fixed_result(case);
        expected_bytes.extend(value.to_le_bytes());
        expected_bytes.extend(failure.to_le_bytes());
    }

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
        compilation_options: wgpu::PipelineCompilationOptions::default(),
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

    assert_eq!(actual_bytes, expected_bytes);
}
