use moria::{
    canonical::math::{FixedI32, WideI64, floor_div, floor_shift_right},
    facade::CanonicalFailure,
};

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
