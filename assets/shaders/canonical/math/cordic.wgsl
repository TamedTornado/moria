// TECH-071's exact table-driven 32-step CORDIC recurrence. No floating-point
// type, transcendental instruction, or implementation-defined signed shift is
// used in this canonical source.

struct CordicWide { low: u32, high: u32, }
struct CordicResult { sin_q30: i32, cos_q30: i32, }

const CORDIC_GAIN_INVERSE_Q61 = CordicWide(0x086bcb4du, 0x136e9db5u);
const CORDIC_ATAN_TURNS_Q62 = array<CordicWide, 32>(
    CordicWide(0x00000000u, 0x08000000u), CordicWide(0x677cc21au, 0x04b90147u),
    CordicWide(0xd7b8e7a3u, 0x027ece16u), CordicWide(0x07776687u, 0x01444475u),
    CordicWide(0xc39626bbu, 0x00a2c350u), CordicWide(0x5641189eu, 0x005175f8u),
    CordicWide(0x970a098au, 0x0028bd87u), CordicWide(0x447510acu, 0x00145f15u),
    CordicWide(0xd1b430ceu, 0x000a2f94u), CordicWide(0xaecc2aceu, 0x000517cbu),
    CordicWide(0x00246e9fu, 0x00028be6u), CordicWide(0x052a032eu, 0x000145f3u),
    CordicWide(0x8337fb18u, 0x0000a2f9u), CordicWide(0xc1b05cbdu, 0x0000517cu),
    CordicWide(0x60daba44u, 0x000028beu), CordicWide(0x306dae9fu, 0x0000145fu),
    CordicWide(0x9836e17fu, 0x00000a2fu), CordicWide(0xcc1b7205u, 0x00000517u),
    CordicWide(0xe60db92bu, 0x0000028bu), CordicWide(0xf306dc9bu, 0x00000145u),
    CordicWide(0xf9836e4eu, 0x000000a2u), CordicWide(0x7cc1b727u, 0x00000051u),
    CordicWide(0xbe60db94u, 0x00000028u), CordicWide(0x5f306dcau, 0x00000014u),
    CordicWide(0x2f9836e5u, 0x0000000au), CordicWide(0x17cc1b72u, 0x00000005u),
    CordicWide(0x8be60db9u, 0x00000002u), CordicWide(0x45f306ddu, 0x00000001u),
    CordicWide(0xa2f9836eu, 0x00000000u), CordicWide(0x517cc1b7u, 0x00000000u),
    CordicWide(0x28be60dcu, 0x00000000u), CordicWide(0x145f306eu, 0x00000000u),
);

fn cordic_negative(value: CordicWide) -> bool { return (value.high & 0x80000000u) != 0u; }
fn cordic_add(left: CordicWide, right: CordicWide) -> CordicWide {
    let low = left.low + right.low;
    return CordicWide(low, left.high + right.high + select(0u, 1u, low < left.low));
}
fn cordic_not(value: CordicWide) -> CordicWide { return CordicWide(~value.low, ~value.high); }
fn cordic_neg(value: CordicWide) -> CordicWide { return cordic_add(cordic_not(value), CordicWide(1u, 0u)); }
fn cordic_sub(left: CordicWide, right: CordicWide) -> CordicWide { return cordic_add(left, cordic_neg(right)); }
fn cordic_floor_shift_right(value: CordicWide, shift: u32) -> CordicWide {
    let fill = select(0u, 0xffffffffu, cordic_negative(value));
    if (shift == 0u) { return value; }
    if (shift < 32u) {
        return CordicWide((value.low >> shift) | (value.high << (32u - shift)), bitcast<u32>(bitcast<i32>(value.high) >> shift));
    }
    if (shift < 64u) {
        return CordicWide(bitcast<u32>(bitcast<i32>(value.high) >> (shift - 32u)), fill);
    }
    return CordicWide(fill, fill);
}
fn cordic_q61_to_q30(value: CordicWide) -> i32 {
    let floor_quotient = cordic_floor_shift_right(value, 31u);
    let fraction = value.low & 0x7fffffffu;
    let increment = fraction > 0x40000000u || (fraction == 0x40000000u && (floor_quotient.low & 1u) != 0u);
    return bitcast<i32>(floor_quotient.low + select(0u, 1u, increment));
}

fn cordic_sine_cosine_q30(angle: u32) -> CordicResult {
    let base_quadrant = angle >> 30u;
    let fraction = angle & 0x3fffffffu;
    let upper_center = fraction >= 0x20000000u;
    let quadrant = (base_quadrant + select(0u, 1u, upper_center)) & 3u;
    let residual_word = bitcast<i32>(select(fraction, fraction - 0x40000000u, upper_center));
    var x = CORDIC_GAIN_INVERSE_Q61;
    var y = CordicWide(0u, 0u);
    var z = CordicWide(bitcast<u32>(residual_word) << 30u, bitcast<u32>(residual_word >> 2));
    for (var index = 0u; index < 32u; index = index + 1u) {
        let before_x = x;
        let before_y = y;
        let before_z = z;
        let shifted_x = cordic_floor_shift_right(before_x, index);
        let shifted_y = cordic_floor_shift_right(before_y, index);
        if (cordic_negative(before_z)) {
            x = cordic_add(before_x, shifted_y);
            y = cordic_sub(before_y, shifted_x);
            z = cordic_add(before_z, CORDIC_ATAN_TURNS_Q62[index]);
        } else {
            x = cordic_sub(before_x, shifted_y);
            y = cordic_add(before_y, shifted_x);
            z = cordic_sub(before_z, CORDIC_ATAN_TURNS_Q62[index]);
        }
    }
    let cosine = cordic_q61_to_q30(x);
    let sine = cordic_q61_to_q30(y);
    if (quadrant == 0u) { return CordicResult(sine, cosine); }
    if (quadrant == 1u) { return CordicResult(cosine, -sine); }
    if (quadrant == 2u) { return CordicResult(-sine, -cosine); }
    return CordicResult(-cosine, sine);
}
