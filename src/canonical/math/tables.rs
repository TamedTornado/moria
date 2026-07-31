//! Checked-in constants for the TECH-071 CORDIC recurrence.
//!
//! These are generated from the specified real values with ties-to-even
//! rounding by the independent qualification fixture generator.  They are
//! data, not an invitation to evaluate transcendental functions at runtime.

/// The Q2.61 inverse gain for the fixed 32-step CORDIC recurrence.
pub(crate) const CORDIC_GAIN_INVERSE_Q61: i64 = 1_400_229_935_014_726_477;

/// The 32 Q2.62 arctangent entries, in increasing iteration order.
pub(crate) const CORDIC_ATAN_TURNS_Q62: [i64; 32] = [
    576_460_752_303_423_488,
    340_304_653_033_718_298,
    179_807_632_645_220_259,
    91_273_161_881_380_487,
    45_813_697_873_323_707,
    22_929_182_573_009_054,
    11_467_389_120_678_282,
    5_734_044_481_687_724,
    2_867_065_987_018_958,
    1_433_538_461_969_102,
    716_769_914_547_871,
    358_385_042_719_534,
    179_192_532_040_472,
    89_596_267_355_325,
    44_798_133_844_548,
    22_399_066_943_135,
    11_199_533_474_175,
    5_599_766_737_413,
    2_799_883_368_747,
    1_399_941_684_379,
    699_970_842_190,
    349_985_421_095,
    174_992_710_548,
    87_496_355_274,
    43_748_177_637,
    21_874_088_818,
    10_937_044_409,
    5_468_522_205,
    2_734_261_102,
    1_367_130_551,
    683_565_276,
    341_782_638,
];
