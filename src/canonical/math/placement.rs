//! Public runtime dispatch for the frozen canonical placement format.

use super::fixed::FixedI32;
use crate::facade::CanonicalFailure;

macro_rules! dispatch {
    ($format:expr, $left:expr, $right:expr, $operation:ident) => {{
        macro_rules! call {
            ($f:literal) => {
                FixedI32::<$f>::try_from_raw($left.0)?
                    .$operation(FixedI32::try_from_raw($right.0)?)
                    .map(|value| PlacementScalar(value.raw()))
            };
        }
        match $format.0 {
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
            _ => Err(CanonicalFailure::InvalidFixedFormat),
        }
    }};
}

macro_rules! dispatch_unary {
    ($format:expr, $value:expr, $operation:ident) => {{
        macro_rules! call {
            ($f:literal) => {
                FixedI32::<$f>::try_from_raw($value.0)?
                    .$operation()
                    .map(|value| PlacementScalar(value.raw()))
            };
        }
        match $format.0 {
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
            _ => Err(CanonicalFailure::InvalidFixedFormat),
        }
    }};
}

/// A validated fractional-bit split for one canonical world's placement values.
///
/// The split is frozen at genesis and must be supplied unchanged for every
/// placement operation in that world.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlacementFixedFormat(u8);

impl PlacementFixedFormat {
    /// Validates a canonical placement split.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::InvalidFixedFormat`] when `fractional_bits`
    /// exceeds 16.
    ///
    /// ```
    /// use moria::canonical::PlacementFixedFormat;
    /// assert!(PlacementFixedFormat::try_new(16).is_ok());
    /// ```
    pub const fn try_new(fractional_bits: u8) -> Result<Self, CanonicalFailure> {
        if fractional_bits > 16 {
            Err(CanonicalFailure::InvalidFixedFormat)
        } else {
            Ok(Self(fractional_bits))
        }
    }

    /// Returns the frozen fractional-bit split.
    #[must_use]
    pub const fn fractional_bits(self) -> u8 {
        self.0
    }
}

/// A raw signed scalar in the world's validated [`PlacementFixedFormat`].
///
/// The raw word has no implicit unit conversion; callers retain and pass the
/// matching format explicitly, so a scalar cannot silently cross world formats.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlacementScalar(i32);

impl PlacementScalar {
    /// Creates a placement scalar from its canonical raw word.
    ///
    /// ```
    /// use moria::canonical::PlacementScalar;
    /// assert_eq!(PlacementScalar::from_raw(7).raw(), 7);
    /// ```
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    /// Returns the canonical raw word.
    #[must_use]
    pub const fn raw(self) -> i32 {
        self.0
    }

    /// Adds two placement scalars exactly.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::ArithmeticOverflow`] instead of saturating.
    pub fn try_add(
        self,
        rhs: Self,
        format: PlacementFixedFormat,
    ) -> Result<Self, CanonicalFailure> {
        dispatch!(format, self, rhs, try_add)
    }

    /// Subtracts two placement scalars exactly.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::ArithmeticOverflow`] instead of saturating.
    pub fn try_sub(
        self,
        rhs: Self,
        format: PlacementFixedFormat,
    ) -> Result<Self, CanonicalFailure> {
        dispatch!(format, self, rhs, try_sub)
    }

    /// Negates this scalar exactly.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::ArithmeticOverflow`] for `i32::MIN`.
    pub fn try_neg(self, format: PlacementFixedFormat) -> Result<Self, CanonicalFailure> {
        dispatch_unary!(format, self, try_neg)
    }

    /// Returns the exact representable absolute magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::ArithmeticOverflow`] for `i32::MIN`.
    pub fn try_abs(self, format: PlacementFixedFormat) -> Result<Self, CanonicalFailure> {
        dispatch_unary!(format, self, try_abs)
    }

    /// Multiplies two placement scalars using the world's frozen format.
    ///
    /// # Errors
    ///
    /// Returns the stable arithmetic failure when the exact reduced result is
    /// not representable; this operation never saturates.
    ///
    /// ```
    /// use moria::canonical::{PlacementFixedFormat, PlacementScalar};
    /// let format = PlacementFixedFormat::try_new(1).unwrap();
    /// assert_eq!(PlacementScalar::from_raw(1).try_mul(PlacementScalar::from_raw(3), format).unwrap().raw(), 2);
    /// ```
    pub fn try_mul(
        self,
        rhs: Self,
        format: PlacementFixedFormat,
    ) -> Result<Self, CanonicalFailure> {
        dispatch!(format, self, rhs, try_mul)
    }

    /// Divides two placement scalars using the world's frozen format.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::DivisionByZero`] for zero `rhs` and the
    /// stable arithmetic failure for an unrepresentable exact result.
    pub fn try_div(
        self,
        rhs: Self,
        format: PlacementFixedFormat,
    ) -> Result<Self, CanonicalFailure> {
        dispatch!(format, self, rhs, try_div)
    }

    /// Returns the nearest ties-even fixed-point square root.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::NegativeSquareRoot`] for negative input.
    pub fn try_sqrt(self, format: PlacementFixedFormat) -> Result<Self, CanonicalFailure> {
        dispatch_unary!(format, self, try_sqrt)
    }

    /// Narrows with ties-to-even right-shift reduction.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::InvalidShift`] above 31.
    pub fn try_narrow(
        self,
        shift: u8,
        format: PlacementFixedFormat,
    ) -> Result<i32, CanonicalFailure> {
        let _ = format;
        FixedI32::<0>::try_from_raw(self.0)?.try_narrow(shift)
    }

    /// Divides raw integers toward negative infinity.
    ///
    /// # Errors
    ///
    /// Returns the stable divide-by-zero or overflow failure.
    pub fn try_floor_div(self, rhs: Self) -> Result<Self, CanonicalFailure> {
        super::fixed::floor_div(self.0, rhs.0).map(Self)
    }

    /// Shifts right toward negative infinity.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailure::InvalidShift`] above 31.
    pub fn try_floor_shift_right(self, shift: u8) -> Result<Self, CanonicalFailure> {
        super::fixed::floor_shift_right(self.0, shift).map(Self)
    }
}
