//! Integer CORDIC and Q1.30 axis normalization specified by TECH-071.

use crate::facade::CanonicalFailure;

use super::tables::{CORDIC_ATAN_TURNS_Q62, CORDIC_GAIN_INVERSE_Q61};

const Q30_ONE: i32 = 1_i32 << 30;

/// One retained state of the normative simultaneous CORDIC recurrence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CordicIteration {
    x_q61: i64,
    y_q61: i64,
    residual_q62: i64,
}

impl CordicIteration {
    #[cfg(test)]
    pub(crate) const fn new(x_q61: i64, y_q61: i64, residual_q62: i64) -> Self {
        Self {
            x_q61,
            y_q61,
            residual_q62,
        }
    }
}

/// Evaluates the fixed 32-step turn recurrence and returns `(sin, cos)` Q1.30.
pub(crate) fn sine_cosine_q30(angle: u32) -> Result<(i32, i32), CanonicalFailure> {
    let q_unwrapped = (u64::from(angle) + 0x2000_0000) >> 30;
    let quadrant = q_unwrapped & 3;
    let residual = i64::from(angle)
        - i64::try_from(q_unwrapped << 30).map_err(|_| CanonicalFailure::ArithmeticOverflow)?;
    let iterations = cordic_from_residual(
        residual
            .checked_shl(30)
            .ok_or(CanonicalFailure::ArithmeticOverflow)?,
    )?;
    let final_state = iterations[31];
    let cosine = narrow_q61_to_q30(final_state.x_q61)?;
    let sine = narrow_q61_to_q30(final_state.y_q61)?;
    match quadrant {
        0 => Ok((sine, cosine)),
        1 => Ok((
            cosine,
            sine.checked_neg()
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
        )),
        2 => Ok((
            sine.checked_neg()
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
            cosine
                .checked_neg()
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
        )),
        3 => Ok((
            cosine
                .checked_neg()
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
            sine,
        )),
        _ => Err(CanonicalFailure::ArithmeticOverflow),
    }
}

/// Retains every state so qualification can compare the exact recurrence.
#[cfg(test)]
pub(crate) fn cordic_iterations(angle: u32) -> Result<[CordicIteration; 32], CanonicalFailure> {
    let q_unwrapped = (u64::from(angle) + 0x2000_0000) >> 30;
    let residual = i64::from(angle)
        - i64::try_from(q_unwrapped << 30).map_err(|_| CanonicalFailure::ArithmeticOverflow)?;
    cordic_from_residual(
        residual
            .checked_shl(30)
            .ok_or(CanonicalFailure::ArithmeticOverflow)?,
    )
}

fn cordic_from_residual(mut residual_q62: i64) -> Result<[CordicIteration; 32], CanonicalFailure> {
    let mut x_q61 = CORDIC_GAIN_INVERSE_Q61;
    let mut y_q61 = 0_i64;
    let mut states = [CordicIteration {
        x_q61: 0,
        y_q61: 0,
        residual_q62: 0,
    }; 32];

    for (index, atan_q62) in CORDIC_ATAN_TURNS_Q62.into_iter().enumerate() {
        // These snapshots make the recurrence simultaneous rather than an
        // order-dependent sequence of component assignments.
        let x_before = x_q61;
        let y_before = y_q61;
        let residual_before = residual_q62;
        let shift = u32::try_from(index).map_err(|_| CanonicalFailure::InvalidShift)?;
        let shifted_x = floor_div_power_of_two(x_before, shift)?;
        let shifted_y = floor_div_power_of_two(y_before, shift)?;
        if residual_before >= 0 {
            x_q61 = x_before
                .checked_sub(shifted_y)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?;
            y_q61 = y_before
                .checked_add(shifted_x)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?;
            residual_q62 = residual_before
                .checked_sub(atan_q62)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?;
        } else {
            x_q61 = x_before
                .checked_add(shifted_y)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?;
            y_q61 = y_before
                .checked_sub(shifted_x)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?;
            residual_q62 = residual_before
                .checked_add(atan_q62)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?;
        }
        states[index] = CordicIteration {
            x_q61,
            y_q61,
            residual_q62,
        };
    }
    Ok(states)
}

fn floor_div_power_of_two(value: i64, shift: u32) -> Result<i64, CanonicalFailure> {
    if shift > 62 {
        return Err(CanonicalFailure::InvalidShift);
    }
    Ok(value >> shift)
}

fn narrow_q61_to_q30(value: i64) -> Result<i32, CanonicalFailure> {
    let divisor = 1_i64 << 31;
    let quotient = value / divisor;
    let remainder = value % divisor;
    let magnitude = remainder.unsigned_abs();
    let doubled = magnitude
        .checked_mul(2)
        .ok_or(CanonicalFailure::ArithmeticOverflow)?;
    let rounded = if doubled < divisor as u64 || (doubled == divisor as u64 && quotient & 1 == 0) {
        quotient
    } else if value < 0 {
        quotient
            .checked_sub(1)
            .ok_or(CanonicalFailure::ArithmeticOverflow)?
    } else {
        quotient
            .checked_add(1)
            .ok_or(CanonicalFailure::ArithmeticOverflow)?
    };
    i32::try_from(rounded).map_err(|_| CanonicalFailure::Nonrepresentable)
}

/// Normalizes an exact signed raw placement vector to signed Q1.30.
pub(crate) fn normalize_axis_q30(axis: [i32; 3]) -> Result<[i32; 3], CanonicalFailure> {
    let magnitudes = axis.map(|component| i64::from(component).unsigned_abs());
    let norm = magnitudes.iter().try_fold(0_u64, |sum, magnitude| {
        let square = magnitude
            .checked_mul(*magnitude)
            .ok_or(CanonicalFailure::UnrepresentableAxis)?;
        sum.checked_add(square)
            .ok_or(CanonicalFailure::UnrepresentableAxis)
    })?;
    if norm == 0 {
        return Err(CanonicalFailure::ZeroAxis);
    }
    normalize_axis_with_norm(axis, magnitudes, norm)
}

fn normalize_axis_with_norm(
    axis: [i32; 3],
    magnitudes: [u64; 3],
    norm: u64,
) -> Result<[i32; 3], CanonicalFailure> {
    if norm == 0 {
        return Err(CanonicalFailure::UnrepresentableAxis);
    }
    let mut normalized = [0_i32; 3];
    for index in 0..3 {
        let magnitude = magnitudes[index];
        let target = u128::from(magnitude)
            .checked_mul(u128::from(magnitude))
            .and_then(|value| value.checked_shl(60))
            .ok_or(CanonicalFailure::UnrepresentableAxis)?;
        let mut low = 0_u64;
        let mut high = u64::try_from(Q30_ONE).map_err(|_| CanonicalFailure::UnrepresentableAxis)?;
        for _ in 0..31 {
            let candidate = (low + high).div_ceil(2);
            let candidate_square = u128::from(candidate) * u128::from(candidate);
            if candidate_square * u128::from(norm) <= target {
                low = candidate;
            } else {
                high = candidate - 1;
            }
        }
        let chosen =
            if low == u64::try_from(Q30_ONE).map_err(|_| CanonicalFailure::UnrepresentableAxis)? {
                low
            } else {
                let left = target
                    .checked_mul(4)
                    .ok_or(CanonicalFailure::UnrepresentableAxis)?;
                let midpoint = low
                    .checked_mul(2)
                    .and_then(|value| value.checked_add(1))
                    .ok_or(CanonicalFailure::UnrepresentableAxis)?;
                let right = u128::from(midpoint)
                    .checked_mul(u128::from(midpoint))
                    .and_then(|value| value.checked_mul(u128::from(norm)))
                    .ok_or(CanonicalFailure::UnrepresentableAxis)?;
                if left < right || (left == right && low & 1 == 0) {
                    low
                } else {
                    low.checked_add(1)
                        .ok_or(CanonicalFailure::UnrepresentableAxis)?
                }
            };
        let signed = i32::try_from(chosen).map_err(|_| CanonicalFailure::UnrepresentableAxis)?;
        normalized[index] = if axis[index] < 0 {
            signed
                .checked_neg()
                .ok_or(CanonicalFailure::UnrepresentableAxis)?
        } else {
            signed
        };
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrupted_nonzero_axis_norm_fails_closed() {
        assert_eq!(
            normalize_axis_with_norm([1, 0, 0], [1, 0, 0], 0),
            Err(CanonicalFailure::UnrepresentableAxis)
        );
    }
}
