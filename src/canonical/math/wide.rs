//! CPU mirror of the two-word signed-wide shader ABI.

use crate::facade::CanonicalFailure;

/// A signed two's-complement 64-bit value represented as little-endian words.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct WideI64 {
    /// Least-significant word.
    pub low: u32,
    /// Most-significant signed word, stored as raw bits.
    pub high: u32,
}

impl WideI64 {
    /// Encodes a signed CPU value in the shader's two-word wire form.
    #[must_use]
    pub const fn from_i64(value: i64) -> Self {
        let bits = value as u64;
        Self {
            low: bits as u32,
            high: (bits >> 32) as u32,
        }
    }

    /// Decodes this two-word two's-complement value.
    #[must_use]
    pub const fn to_i64(self) -> i64 {
        (((self.high as u64) << 32) | self.low as u64) as i64
    }

    /// Returns the exact eight-byte little-endian wire representation.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        let low = self.low.to_le_bytes();
        let high = self.high.to_le_bytes();
        [
            low[0], low[1], low[2], low[3], high[0], high[1], high[2], high[3],
        ]
    }

    /// Compares signed-wide values using their two's-complement interpretation.
    #[must_use]
    pub fn cmp_signed(self, rhs: Self) -> core::cmp::Ordering {
        self.to_i64().cmp(&rhs.to_i64())
    }

    /// Adds signed-wide values without saturation.
    pub fn checked_add(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        self.to_i64()
            .checked_add(rhs.to_i64())
            .map(Self::from_i64)
            .ok_or(CanonicalFailure::ArithmeticOverflow)
    }

    /// Subtracts signed-wide values without saturation.
    pub fn checked_sub(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        self.to_i64()
            .checked_sub(rhs.to_i64())
            .map(Self::from_i64)
            .ok_or(CanonicalFailure::ArithmeticOverflow)
    }

    /// Multiplies signed-wide values without saturation.
    pub fn checked_mul(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        self.to_i64()
            .checked_mul(rhs.to_i64())
            .map(Self::from_i64)
            .ok_or(CanonicalFailure::ArithmeticOverflow)
    }

    /// Divides toward negative infinity.
    pub fn floor_div(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        let divisor = rhs.to_i64();
        if divisor == 0 {
            return Err(CanonicalFailure::DivisionByZero);
        }
        let dividend = self.to_i64();
        let quotient = dividend
            .checked_div(divisor)
            .ok_or(CanonicalFailure::ArithmeticOverflow)?;
        let remainder = dividend % divisor;
        if remainder != 0 && (dividend < 0) != (divisor < 0) {
            quotient
                .checked_sub(1)
                .map(Self::from_i64)
                .ok_or(CanonicalFailure::ArithmeticOverflow)
        } else {
            Ok(Self::from_i64(quotient))
        }
    }

    /// Shifts right toward negative infinity.
    pub fn floor_shift_right(self, shift: u8) -> Result<Self, CanonicalFailure> {
        if shift > 63 {
            return Err(CanonicalFailure::InvalidShift);
        }
        Ok(Self::from_i64(self.to_i64() >> shift))
    }
}
