// TECH-071 fixed-point helpers. `low` then `high` is the canonical wide ABI.
override FRACTIONAL_BITS: u32 = 0u;

struct WideI64 {
    low: u32,
    high: u32,
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
    if (left_negative != right_negative) {
        return select(1, -1, left_negative);
    }
    return wide_cmp_unsigned(left, right);
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

// Performs a fixed 64-step unsigned long division. The caller rejects zero
// divisors and applies the sign and ties-to-even rounding policy separately.
fn wide_div_unsigned(dividend: WideI64, divisor: WideI64) -> WideI64 {
    var quotient = WideI64(0u, 0u);
    var remainder = WideI64(0u, 0u);
    var bit = 64u;
    loop {
        if (bit == 0u) { break; }
        bit = bit - 1u;
        let source = select((dividend.low >> bit) & 1u, (dividend.high >> (bit - 32u)) & 1u, bit >= 32u);
        remainder = wide_add(WideI64(remainder.low << 1u, (remainder.high << 1u) | (remainder.low >> 31u)), WideI64(source, 0u));
        if (wide_cmp_unsigned(remainder, divisor) >= 0) {
            remainder = wide_sub(remainder, divisor);
            if (bit < 32u) { quotient.low = quotient.low | (1u << bit); }
            if (bit >= 32u) { quotient.high = quotient.high | (1u << (bit - 32u)); }
        }
    }
    return quotient;
}
