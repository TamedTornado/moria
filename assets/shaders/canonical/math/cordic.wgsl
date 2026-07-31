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

// The axis path deliberately represents every comparison product as four
// little-endian words.  A native i64/u64 shader type is not a baseline
// dependency, and two words cannot hold the Q1.30 normalization comparisons.
struct AxisWide { word0: u32, word1: u32, word2: u32, word3: u32, }
struct AxisProduct { low: u32, high: u32, }
struct AxisAdd { value: CordicWide, overflow: bool, }
struct AxisResult { x: i32, y: i32, z: i32, failure: u32, }

const CORDIC_OK: u32 = 0u;
const CORDIC_ZERO_AXIS: u32 = 18u;
const CORDIC_UNREPRESENTABLE_AXIS: u32 = 19u;
const CORDIC_PARITY_WORDS: u32 = 198u;

fn axis_mul_u32(left: u32, right: u32) -> AxisProduct {
    let left_low = left & 0xffffu;
    let left_high = left >> 16u;
    let right_low = right & 0xffffu;
    let right_high = right >> 16u;
    let product_low = left_low * right_low;
    let middle_first = (product_low >> 16u) + left_low * right_high;
    let carry_first = select(0u, 1u, middle_first < (product_low >> 16u));
    let middle = middle_first + left_high * right_low;
    let carry_middle = carry_first + select(0u, 1u, middle < middle_first);
    return AxisProduct(
        (product_low & 0xffffu) | (middle << 16u),
        left_high * right_high + (middle >> 16u) + (carry_middle << 16u),
    );
}

fn axis_mul_u64(left: CordicWide, right: CordicWide) -> AxisWide {
    let p00 = axis_mul_u32(left.low, right.low);
    let p01 = axis_mul_u32(left.low, right.high);
    let p10 = axis_mul_u32(left.high, right.low);
    let p11 = axis_mul_u32(left.high, right.high);
    let middle_first = p00.high + p01.low;
    let carry_first = select(0u, 1u, middle_first < p00.high);
    let middle = middle_first + p10.low;
    let carry_middle = carry_first + select(0u, 1u, middle < middle_first);
    let upper_first = p01.high + p10.high;
    let carry_upper_first = select(0u, 1u, upper_first < p01.high);
    let upper_second = upper_first + p11.low;
    let carry_upper_second = carry_upper_first + select(0u, 1u, upper_second < upper_first);
    let upper = upper_second + carry_middle;
    let carry_upper = carry_upper_second + select(0u, 1u, upper < upper_second);
    return AxisWide(p00.low, middle, upper, p11.high + carry_upper);
}

fn axis_cmp(left: AxisWide, right: AxisWide) -> i32 {
    if (left.word3 < right.word3) { return -1; }
    if (left.word3 > right.word3) { return 1; }
    if (left.word2 < right.word2) { return -1; }
    if (left.word2 > right.word2) { return 1; }
    if (left.word1 < right.word1) { return -1; }
    if (left.word1 > right.word1) { return 1; }
    if (left.word0 < right.word0) { return -1; }
    if (left.word0 > right.word0) { return 1; }
    return 0;
}

fn axis_add_u64(left: CordicWide, right: CordicWide) -> AxisAdd {
    let low = left.low + right.low;
    let carry = select(0u, 1u, low < left.low);
    let high_first = left.high + right.high;
    let high_first_carry = high_first < left.high;
    let high = high_first + carry;
    return AxisAdd(CordicWide(low, high), high_first_carry || high < high_first);
}

fn axis_shift_left_60(value: CordicWide) -> AxisWide {
    return AxisWide(0u, value.low << 28u, (value.high << 28u) | (value.low >> 4u), value.high >> 4u);
}

fn axis_shift_left_2(value: AxisWide) -> AxisWide {
    return AxisWide(
        value.word0 << 2u,
        (value.word1 << 2u) | (value.word0 >> 30u),
        (value.word2 << 2u) | (value.word1 >> 30u),
        (value.word3 << 2u) | (value.word2 >> 30u),
    );
}

fn axis_abs(value: i32) -> u32 {
    let bits = bitcast<u32>(value);
    return select(bits, 0u - bits, value < 0);
}

fn axis_normalize_q30(axis: vec3<i32>) -> AxisResult {
    let magnitudes = vec3<u32>(axis_abs(axis.x), axis_abs(axis.y), axis_abs(axis.z));
    let square_x = axis_mul_u32(magnitudes.x, magnitudes.x);
    let square_y = axis_mul_u32(magnitudes.y, magnitudes.y);
    let square_z = axis_mul_u32(magnitudes.z, magnitudes.z);
    let norm_xy = axis_add_u64(CordicWide(square_x.low, square_x.high), CordicWide(square_y.low, square_y.high));
    let norm_xyz = axis_add_u64(norm_xy.value, CordicWide(square_z.low, square_z.high));
    if (norm_xy.overflow || norm_xyz.overflow) {
        return AxisResult(0, 0, 0, CORDIC_UNREPRESENTABLE_AXIS);
    }
    let norm = norm_xyz.value;
    if (norm.low == 0u && norm.high == 0u) { return AxisResult(0, 0, 0, CORDIC_ZERO_AXIS); }
    var result = vec3<i32>(0, 0, 0);
    for (var component = 0u; component < 3u; component = component + 1u) {
        let magnitude = magnitudes[component];
        let square = axis_mul_u32(magnitude, magnitude);
        let threshold = axis_shift_left_60(CordicWide(square.low, square.high));
        var low = 0u;
        var high = 0x40000000u;
        for (var iteration = 0u; iteration < 31u; iteration = iteration + 1u) {
            let candidate = (low + high + 1u) >> 1u;
            let candidate_square = axis_mul_u32(candidate, candidate);
            let candidate_product = axis_mul_u64(CordicWide(candidate_square.low, candidate_square.high), norm);
            if (axis_cmp(candidate_product, threshold) <= 0) { low = candidate; }
            else { high = candidate - 1u; }
        }
        var chosen = low;
        if (low != 0x40000000u) {
            let left = axis_shift_left_2(threshold);
            let midpoint = low * 2u + 1u;
            let midpoint_square = axis_mul_u32(midpoint, midpoint);
            let right = axis_mul_u64(CordicWide(midpoint_square.low, midpoint_square.high), norm);
            let comparison = axis_cmp(left, right);
            if (comparison > 0 || (comparison == 0 && (low & 1u) != 0u)) { chosen = low + 1u; }
        }
        let signed = bitcast<i32>(chosen);
        result[component] = select(signed, -signed, axis[component] < 0);
    }
    return AxisResult(result.x, result.y, result.z, CORDIC_OK);
}

struct CordicParityInput { angle: u32, axis_x: i32, axis_y: i32, axis_z: i32, }
@group(0) @binding(0) var<storage, read> cordic_inputs: array<CordicParityInput>;
@group(0) @binding(1) var<storage, read_write> cordic_outputs: array<u32>;

@compute @workgroup_size(1)
fn cordic_parity(@builtin(global_invocation_id) invocation: vec3<u32>) {
    if (invocation.x >= arrayLength(&cordic_inputs)) { return; }
    let base = invocation.x * CORDIC_PARITY_WORDS;
    if (base + CORDIC_PARITY_WORDS > arrayLength(&cordic_outputs)) { return; }
    let input = cordic_inputs[invocation.x];
    let base_quadrant = input.angle >> 30u;
    let fraction = input.angle & 0x3fffffffu;
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
        let state = base + index * 6u;
        cordic_outputs[state] = x.low;
        cordic_outputs[state + 1u] = x.high;
        cordic_outputs[state + 2u] = y.low;
        cordic_outputs[state + 3u] = y.high;
        cordic_outputs[state + 4u] = z.low;
        cordic_outputs[state + 5u] = z.high;
    }
    // Exercise the production final-remap helper as part of executable parity
    // coverage; the loop above retains the internal state ledger.
    let result = cordic_sine_cosine_q30(input.angle);
    let axis = axis_normalize_q30(vec3<i32>(input.axis_x, input.axis_y, input.axis_z));
    cordic_outputs[base + 192u] = bitcast<u32>(result.sin_q30);
    cordic_outputs[base + 193u] = bitcast<u32>(result.cos_q30);
    cordic_outputs[base + 194u] = bitcast<u32>(axis.x);
    cordic_outputs[base + 195u] = bitcast<u32>(axis.y);
    cordic_outputs[base + 196u] = bitcast<u32>(axis.z);
    cordic_outputs[base + 197u] = axis.failure;
}
