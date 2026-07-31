// TECH-071 fixed-point helpers and differential-parity compute entry point.
// `low` then `high` is the canonical signed-wide ABI.
const FIXED_OK: u32 = 0u;
// Stable CanonicalFailure v1 wire tags. Keep these in lockstep with
// CanonicalFailure::wire_tag().
const FIXED_INVALID_FORMAT: u32 = 7u;
const FIXED_ARITHMETIC_OVERFLOW: u32 = 8u;
const FIXED_DIVISION_BY_ZERO: u32 = 9u;
const FIXED_INVALID_SHIFT: u32 = 10u;
const FIXED_NEGATIVE_SQUARE_ROOT: u32 = 11u;
const FIXED_NONREPRESENTABLE: u32 = 12u;

struct WideI64 {
    low: u32,
    high: u32,
}

struct WideDivision {
    quotient: WideI64,
    remainder: WideI64,
}

fn wide_add(left: WideI64, right: WideI64) -> WideI64 {
    let low = left.low + right.low;
    let carry = select(0u, 1u, low < left.low);
    return WideI64(low, left.high + right.high + carry);
}

fn wide_sub(left: WideI64, right: WideI64) -> WideI64 {
    let low = left.low - right.low;
    let borrow = select(0u, 1u, left.low < right.low);
    return WideI64(low, left.high - right.high - borrow);
}

fn wide_negative(value: WideI64) -> bool {
    return (value.high & 0x80000000u) != 0u;
}

fn wide_not(value: WideI64) -> WideI64 {
    return WideI64(~value.low, ~value.high);
}

fn wide_neg(value: WideI64) -> WideI64 {
    return wide_add(wide_not(value), WideI64(1u, 0u));
}

fn wide_abs(value: WideI64) -> WideI64 {
    if (wide_negative(value)) {
        return wide_neg(value);
    }
    return value;
}

// The 16-bit decomposition produces the complete low 64 product without a
// native i64 dependency. Inputs to canonical fixed multiplication are i32.
fn wide_mul_i32(left: i32, right: i32) -> WideI64 {
    let left_negative = left < 0;
    let right_negative = right < 0;
    let left_bits = bitcast<u32>(left);
    let right_bits = bitcast<u32>(right);
    let left_magnitude = select(left_bits, 0u - left_bits, left_negative);
    let right_magnitude = select(right_bits, 0u - right_bits, right_negative);
    let left_low = left_magnitude & 0xffffu;
    let left_high = left_magnitude >> 16u;
    let right_low = right_magnitude & 0xffffu;
    let right_high = right_magnitude >> 16u;
    let product_low = left_low * right_low;
    let middle = (product_low >> 16u) + left_low * right_high + left_high * right_low;
    let product = WideI64(
        (product_low & 0xffffu) | (middle << 16u),
        left_high * right_high + (middle >> 16u),
    );
    if (left_negative != right_negative) {
        return wide_neg(product);
    }
    return product;
}

fn wide_cmp_unsigned(left: WideI64, right: WideI64) -> i32 {
    if (left.high < right.high) { return -1; }
    if (left.high > right.high) { return 1; }
    if (left.low < right.low) { return -1; }
    if (left.low > right.low) { return 1; }
    return 0;
}

fn wide_cmp_signed(left: WideI64, right: WideI64) -> i32 {
    let left_negative = wide_negative(left);
    let right_negative = wide_negative(right);
    if (left_negative != right_negative) { return select(1, -1, left_negative); }
    return wide_cmp_unsigned(left, right);
}

fn wide_shl(value: WideI64, shift: u32) -> WideI64 {
    if (shift == 0u) { return value; }
    if (shift < 32u) {
        return WideI64(value.low << shift, (value.high << shift) | (value.low >> (32u - shift)));
    }
    if (shift < 64u) { return WideI64(0u, value.low << (shift - 32u)); }
    return WideI64(0u, 0u);
}

fn wide_shr_floor(value: WideI64, shift: u32) -> WideI64 {
    let fill = select(0u, 0xffffffffu, wide_negative(value));
    if (shift == 0u) { return value; }
    if (shift < 32u) {
        return WideI64((value.low >> shift) | (value.high << (32u - shift)), (value.high >> shift) | (fill << (32u - shift)));
    }
    if (shift < 64u) { return WideI64((value.high >> (shift - 32u)) | (fill << (64u - shift)), fill); }
    return WideI64(fill, fill);
}

// Performs a fixed 64-step unsigned long division.
fn wide_div_unsigned(dividend: WideI64, divisor: WideI64) -> WideDivision {
    var quotient = WideI64(0u, 0u);
    var remainder = WideI64(0u, 0u);
    var bit = 64u;
    loop {
        if (bit == 0u) { break; }
        bit = bit - 1u;
        var source = 0u;
        if (bit < 32u) {
            source = (dividend.low >> bit) & 1u;
        } else {
            source = (dividend.high >> (bit - 32u)) & 1u;
        }
        remainder = wide_add(wide_shl(remainder, 1u), WideI64(source, 0u));
        if (wide_cmp_unsigned(remainder, divisor) >= 0) {
            remainder = wide_sub(remainder, divisor);
            if (bit < 32u) { quotient.low = quotient.low | (1u << bit); }
            else { quotient.high = quotient.high | (1u << (bit - 32u)); }
        }
    }
    return WideDivision(quotient, remainder);
}

fn wide_floor_div(left: WideI64, right: WideI64) -> WideI64 {
    let quotient_and_remainder = wide_div_unsigned(wide_abs(left), wide_abs(right));
    var quotient = quotient_and_remainder.quotient;
    if ((wide_negative(left) != wide_negative(right))
        && (quotient_and_remainder.remainder.low != 0u || quotient_and_remainder.remainder.high != 0u)) {
        quotient = wide_add(quotient, WideI64(1u, 0u));
    }
    if (wide_negative(left) != wide_negative(right)) { return wide_neg(quotient); }
    return quotient;
}

fn wide_to_i32(value: WideI64) -> i32 {
    return bitcast<i32>(value.low);
}

fn wide_fits_i32(value: WideI64) -> bool {
    return (value.high == 0u && value.low <= 0x7fffffffu)
        || (value.high == 0xffffffffu && value.low >= 0x80000000u);
}

// Rounds a signed wide integer divided by 2^shift to nearest, ties to even.
fn fixed_round_power_of_two(value: WideI64, shift: u32) -> WideI64 {
    if (shift == 0u) { return value; }
    let negative = wide_negative(value);
    let magnitude = wide_abs(value);
    var quotient = wide_shr_floor(magnitude, shift);
    let remainder = wide_sub(magnitude, wide_shl(quotient, shift));
    let half = WideI64(1u << (shift - 1u), 0u);
    let comparison = wide_cmp_unsigned(remainder, half);
    if (comparison > 0 || (comparison == 0 && (quotient.low & 1u) != 0u)) {
        quotient = wide_add(quotient, WideI64(1u, 0u));
    }
    if (negative) { return wide_neg(quotient); }
    return quotient;
}

fn fixed_add(left: i32, right: i32) -> vec2<u32> {
    let result = left + right;
    if (((left ^ result) & (right ^ result)) < 0) {
        return vec2<u32>(0u, FIXED_ARITHMETIC_OVERFLOW);
    }
    return vec2<u32>(bitcast<u32>(result), FIXED_OK);
}

fn fixed_sub(left: i32, right: i32) -> vec2<u32> {
    let result = left - right;
    if (((left ^ right) & (left ^ result)) < 0) {
        return vec2<u32>(0u, FIXED_ARITHMETIC_OVERFLOW);
    }
    return vec2<u32>(bitcast<u32>(result), FIXED_OK);
}

fn fixed_neg(value: i32) -> vec2<u32> {
    if (value == -2147483648) { return vec2<u32>(0u, FIXED_ARITHMETIC_OVERFLOW); }
    return vec2<u32>(bitcast<u32>(-value), FIXED_OK);
}

fn fixed_abs(value: i32) -> vec2<u32> {
    if (value < 0) { return fixed_neg(value); }
    return vec2<u32>(bitcast<u32>(value), FIXED_OK);
}

fn fixed_mul(left: i32, right: i32, fractional_bits: u32) -> vec2<u32> {
    let result = fixed_round_power_of_two(wide_mul_i32(left, right), fractional_bits);
    if (!wide_fits_i32(result)) { return vec2<u32>(0u, FIXED_NONREPRESENTABLE); }
    return vec2<u32>(bitcast<u32>(wide_to_i32(result)), FIXED_OK);
}

fn fixed_div(left: i32, right: i32, fractional_bits: u32) -> vec2<u32> {
    if (right == 0) { return vec2<u32>(0u, FIXED_DIVISION_BY_ZERO); }
    let numerator = wide_shl(WideI64(bitcast<u32>(left), select(0u, 0xffffffffu, left < 0)), fractional_bits);
    let numerator_negative = wide_negative(numerator);
    let denominator_negative = right < 0;
    let quotient_and_remainder = wide_div_unsigned(wide_abs(numerator), WideI64(select(bitcast<u32>(right), 0u - bitcast<u32>(right), denominator_negative), 0u));
    var quotient = quotient_and_remainder.quotient;
    let complement = wide_sub(WideI64(select(bitcast<u32>(right), 0u - bitcast<u32>(right), denominator_negative), 0u), quotient_and_remainder.remainder);
    let comparison = wide_cmp_unsigned(quotient_and_remainder.remainder, complement);
    if (comparison > 0 || (comparison == 0 && (quotient.low & 1u) != 0u)) {
        quotient = wide_add(quotient, WideI64(1u, 0u));
    }
    if (numerator_negative != denominator_negative) { quotient = wide_neg(quotient); }
    if (!wide_fits_i32(quotient)) { return vec2<u32>(0u, FIXED_NONREPRESENTABLE); }
    return vec2<u32>(bitcast<u32>(wide_to_i32(quotient)), FIXED_OK);
}

fn fixed_sqrt(value: i32, fractional_bits: u32) -> vec2<u32> {
    if (value < 0) { return vec2<u32>(0u, FIXED_NEGATIVE_SQUARE_ROOT); }
    let radicand = wide_shl(WideI64(bitcast<u32>(value), 0u), fractional_bits);
    var lower = 0u;
    var bit = 32u;
    loop {
        if (bit == 0u) { break; }
        bit = bit - 1u;
        let candidate = lower | (1u << bit);
        if (wide_cmp_unsigned(wide_mul_i32(bitcast<i32>(candidate), bitcast<i32>(candidate)), radicand) <= 0) {
            lower = candidate;
        }
    }
    let lower_squared = wide_mul_i32(bitcast<i32>(lower), bitcast<i32>(lower));
    let upper = lower + 1u;
    let upper_squared = wide_mul_i32(bitcast<i32>(upper), bitcast<i32>(upper));
    let lower_distance = wide_sub(radicand, lower_squared);
    let upper_distance = wide_sub(upper_squared, radicand);
    let comparison = wide_cmp_unsigned(lower_distance, upper_distance);
    let chosen = select(upper, lower, comparison < 0 || (comparison == 0 && (lower & 1u) == 0u));
    return vec2<u32>(chosen, FIXED_OK);
}

fn fixed_narrow(value: i32, shift: u32) -> vec2<u32> {
    if (shift > 31u) { return vec2<u32>(0u, FIXED_INVALID_SHIFT); }
    let result = fixed_round_power_of_two(WideI64(bitcast<u32>(value), select(0u, 0xffffffffu, value < 0)), shift);
    if (!wide_fits_i32(result)) { return vec2<u32>(0u, FIXED_NONREPRESENTABLE); }
    return vec2<u32>(bitcast<u32>(wide_to_i32(result)), FIXED_OK);
}

fn fixed_floor_div(left: i32, right: i32) -> vec2<u32> {
    if (right == 0) { return vec2<u32>(0u, FIXED_DIVISION_BY_ZERO); }
    if (left == -2147483648 && right == -1) {
        return vec2<u32>(0u, FIXED_ARITHMETIC_OVERFLOW);
    }
    var quotient = left / right;
    let remainder = left % right;
    if (remainder != 0 && ((left < 0) != (right < 0))) { quotient = quotient - 1; }
    return vec2<u32>(bitcast<u32>(quotient), FIXED_OK);
}

fn fixed_floor_shift(value: i32, shift: u32) -> vec2<u32> {
    if (shift > 31u) { return vec2<u32>(0u, FIXED_INVALID_SHIFT); }
    return vec2<u32>(bitcast<u32>(value >> shift), FIXED_OK);
}

struct FixedParityInput {
    left: i32,
    right: i32,
    fractional_bits: u32,
    shift: u32,
    operation: u32,
}

struct FixedParityOutput {
    value: i32,
    high: u32,
    failure: u32,
}

@group(0) @binding(0) var<storage, read> fixed_inputs: array<FixedParityInput>;
@group(0) @binding(1) var<storage, read_write> fixed_outputs: array<FixedParityOutput>;

@compute @workgroup_size(1)
fn fixed_parity(@builtin(global_invocation_id) invocation: vec3<u32>) {
    if (invocation.x >= arrayLength(&fixed_inputs)) { return; }
    let input = fixed_inputs[invocation.x];
    if (input.fractional_bits > 16u) {
        fixed_outputs[invocation.x] = FixedParityOutput(0, 0u, FIXED_INVALID_FORMAT);
        return;
    }
    var result = vec2<u32>(0u, FIXED_INVALID_SHIFT);
    var wide_result = WideI64(0u, 0u);
    var is_wide = false;
    switch input.operation {
        case 0u: { result = fixed_add(input.left, input.right); }
        case 1u: { result = fixed_mul(input.left, input.right, input.fractional_bits); }
        case 2u: { result = fixed_div(input.left, input.right, input.fractional_bits); }
        case 3u: { result = fixed_sqrt(input.left, input.fractional_bits); }
        case 4u: { result = fixed_narrow(input.left, input.shift); }
        case 5u: { result = fixed_sub(input.left, input.right); }
        case 6u: { result = fixed_neg(input.left); }
        case 7u: { result = fixed_abs(input.left); }
        case 8u: { result = fixed_floor_div(input.left, input.right); }
        case 9u: { result = fixed_floor_shift(input.left, input.shift); }
        case 10u: {
            wide_result = wide_add(
                WideI64(bitcast<u32>(input.left), select(0u, 0xffffffffu, input.left < 0)),
                WideI64(bitcast<u32>(input.right), select(0u, 0xffffffffu, input.right < 0)),
            );
            is_wide = true;
        }
        case 11u: {
            wide_result = wide_sub(
                WideI64(bitcast<u32>(input.left), select(0u, 0xffffffffu, input.left < 0)),
                WideI64(bitcast<u32>(input.right), select(0u, 0xffffffffu, input.right < 0)),
            );
            is_wide = true;
        }
        case 12u: { wide_result = wide_mul_i32(input.left, input.right); is_wide = true; }
        case 13u: {
            wide_result = wide_shr_floor(wide_mul_i32(input.left, input.right), input.shift);
            is_wide = true;
        }
        case 14u: {
            if (input.right == 0) { result = vec2<u32>(0u, FIXED_DIVISION_BY_ZERO); }
            else if (input.left == -2147483648 && input.right == -1) {
                result = vec2<u32>(0u, FIXED_ARITHMETIC_OVERFLOW);
            } else {
                wide_result = wide_floor_div(
                    WideI64(bitcast<u32>(input.left), select(0u, 0xffffffffu, input.left < 0)),
                    WideI64(bitcast<u32>(input.right), select(0u, 0xffffffffu, input.right < 0)),
                );
                is_wide = true;
            }
        }
        case 15u: {
            result = vec2<u32>(bitcast<u32>(wide_cmp_signed(
                WideI64(bitcast<u32>(input.left), select(0u, 0xffffffffu, input.left < 0)),
                WideI64(bitcast<u32>(input.right), select(0u, 0xffffffffu, input.right < 0)),
            )), FIXED_OK);
        }
        default: {}
    }
    if (is_wide) {
        fixed_outputs[invocation.x] = FixedParityOutput(bitcast<i32>(wide_result.low), wide_result.high, FIXED_OK);
        return;
    }
    fixed_outputs[invocation.x] = FixedParityOutput(
        bitcast<i32>(result.x),
        select(0u, 0xffffffffu, bitcast<i32>(result.x) < 0),
        result.y,
    );
}
