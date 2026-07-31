//! Fixed-point scalar operations specified by TECH-071.

use crate::facade::CanonicalFailure;

/// A signed fixed-point raw value with a validated fractional-bit split.
///
/// `FRACTIONAL_BITS` must be in `0..=16`. Arithmetic never saturates: an
/// unrepresentable result returns [`CanonicalFailure`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FixedI32<const FRACTIONAL_BITS: u8>(i32);

impl<const FRACTIONAL_BITS: u8> FixedI32<FRACTIONAL_BITS> {
    /// Creates a fixed-point value from its canonical raw integer.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::InvalidFixedFormat`] unless
    /// `FRACTIONAL_BITS` is in `0..=16`.
    pub const fn try_from_raw(raw: i32) -> Result<Self, CanonicalFailure> {
        if FRACTIONAL_BITS > 16 {
            Err(CanonicalFailure::InvalidFixedFormat)
        } else {
            Ok(Self(raw))
        }
    }

    /// Returns the canonical signed raw integer.
    #[must_use]
    pub const fn raw(self) -> i32 {
        self.0
    }

    /// Adds two values exactly.
    pub fn try_add(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        Self::try_from_raw(
            self.0
                .checked_add(rhs.0)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
        )
    }

    /// Subtracts two values exactly.
    pub fn try_sub(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        Self::try_from_raw(
            self.0
                .checked_sub(rhs.0)
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
        )
    }

    /// Negates this value exactly.
    pub fn try_neg(self) -> Result<Self, CanonicalFailure> {
        Self::try_from_raw(
            self.0
                .checked_neg()
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
        )
    }

    /// Returns this value's exact absolute raw magnitude when representable.
    pub fn try_abs(self) -> Result<Self, CanonicalFailure> {
        Self::try_from_raw(
            self.0
                .checked_abs()
                .ok_or(CanonicalFailure::ArithmeticOverflow)?,
        )
    }

    /// Multiplies and reduces by `2^FRACTIONAL_BITS` with ties-to-even rounding.
    pub fn try_mul(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        let product = i64::from(self.0) * i64::from(rhs.0);
        Self::from_i64(round_by_power_of_two(product, FRACTIONAL_BITS)?)
    }

    /// Divides and rounds the exact scaled quotient to nearest, ties to even.
    pub fn try_div(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        if rhs.0 == 0 {
            return Err(CanonicalFailure::DivisionByZero);
        }
        let numerator = i64::from(self.0) << FRACTIONAL_BITS;
        Self::from_i64(round_quotient(numerator, i64::from(rhs.0))?)
    }

    /// Computes the nearest fixed-point square root, with ties to even.
    pub fn try_sqrt(self) -> Result<Self, CanonicalFailure> {
        if self.0 < 0 {
            return Err(CanonicalFailure::NegativeSquareRoot);
        }
        let radicand = u64::from(self.0 as u32) << FRACTIONAL_BITS;
        let lower = floor_sqrt(radicand);
        let lower_squared = u64::from(lower) * u64::from(lower);
        let upper = lower
            .checked_add(1)
            .ok_or(CanonicalFailure::ArithmeticOverflow)?;
        let upper_squared = u64::from(upper) * u64::from(upper);
        let lower_distance = radicand - lower_squared;
        let upper_distance = upper_squared - radicand;
        let chosen = if lower_distance < upper_distance {
            lower
        } else if lower_distance > upper_distance {
            upper
        } else if lower & 1 == 0 {
            lower
        } else {
            upper
        };
        Self::from_i64(i64::from(chosen))
    }

    /// Narrows the raw value by a nonnegative right shift using ties-to-even.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::InvalidShift`] for shifts above 31.
    pub fn try_narrow(self, shift: u8) -> Result<i32, CanonicalFailure> {
        round_by_power_of_two(i64::from(self.0), shift).and_then(i32_from_i64)
    }

    fn from_i64(raw: i64) -> Result<Self, CanonicalFailure> {
        Self::try_from_raw(i32_from_i64(raw)?)
    }
}

/// Divides toward negative infinity.
///
/// # Errors
///
/// Returns [`CanonicalFailure::DivisionByZero`] for zero and
/// [`CanonicalFailure::ArithmeticOverflow`] for `i32::MIN / -1`.
pub fn floor_div(value: i32, divisor: i32) -> Result<i32, CanonicalFailure> {
    if divisor == 0 {
        return Err(CanonicalFailure::DivisionByZero);
    }
    value
        .checked_div_euclid(divisor)
        .ok_or(CanonicalFailure::ArithmeticOverflow)
}

/// Shifts right with floor semantics, distinct from ties-to-even narrowing.
///
/// # Errors
///
/// Returns [`CanonicalFailure::InvalidShift`] for shifts above 31.
pub fn floor_shift_right(value: i32, shift: u8) -> Result<i32, CanonicalFailure> {
    if shift > 31 {
        return Err(CanonicalFailure::InvalidShift);
    }
    floor_div(value, 1_i32 << shift)
}

fn i32_from_i64(value: i64) -> Result<i32, CanonicalFailure> {
    i32::try_from(value).map_err(|_| CanonicalFailure::Nonrepresentable)
}

fn round_by_power_of_two(value: i64, shift: u8) -> Result<i64, CanonicalFailure> {
    if shift > 62 {
        return Err(CanonicalFailure::InvalidShift);
    }
    round_quotient(value, 1_i64 << shift)
}

fn round_quotient(numerator: i64, denominator: i64) -> Result<i64, CanonicalFailure> {
    if denominator == 0 {
        return Err(CanonicalFailure::DivisionByZero);
    }
    if numerator == i64::MIN && denominator == -1 {
        return Err(CanonicalFailure::ArithmeticOverflow);
    }
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    let remainder_magnitude = remainder.unsigned_abs();
    let denominator_magnitude = denominator.unsigned_abs();
    let complement = denominator_magnitude - remainder_magnitude;
    if remainder_magnitude < complement || (remainder_magnitude == complement && quotient & 1 == 0)
    {
        return Ok(quotient);
    }
    if (numerator < 0) != (denominator < 0) {
        quotient
            .checked_sub(1)
            .ok_or(CanonicalFailure::ArithmeticOverflow)
    } else {
        quotient
            .checked_add(1)
            .ok_or(CanonicalFailure::ArithmeticOverflow)
    }
}

fn floor_sqrt(radicand: u64) -> u32 {
    let mut root = 0_u32;
    let mut bit = 31_i32;
    while bit >= 0 {
        let candidate = root | (1_u32 << bit);
        if u64::from(candidate) * u64::from(candidate) <= radicand {
            root = candidate;
        }
        bit -= 1;
    }
    root
}
