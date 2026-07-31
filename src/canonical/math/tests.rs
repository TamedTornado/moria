use super::wide::WideI64;
use crate::facade::CanonicalFailure;
use std::{
    future::Future,
    pin::pin,
    sync::mpsc,
    task::{Context, Poll, Waker},
    time::Duration,
};

const FIXED_PARITY_WGSL: &str = include_str!("../../../assets/shaders/canonical/math/fixed.wgsl");

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
        run_gpu_parity(
            &device,
            &queue,
            &generated_fixed_cases(fractional_bits),
            fractional_bits,
        );
    }
    run_gpu_parity(&device, &queue, &generated_wide_cases(), 0);
    run_gpu_parity(&device, &queue, &generated_fixed_cases(0), 17);
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
