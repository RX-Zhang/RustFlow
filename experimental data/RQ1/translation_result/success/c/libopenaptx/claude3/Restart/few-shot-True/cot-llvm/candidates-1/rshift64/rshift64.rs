use std::num::Wrapping;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = Wrapping(1i64 << (shift.wrapping_sub(1) % 64));
    let mask = Wrapping((1i64 << (shift.wrapping_add(1) % 64)).wrapping_sub(1));
    let tmp = Wrapping(value).0.wrapping_add(rounding.0);
    let result = (tmp >> (shift % 64)) as i64;
    let tmp2 = Wrapping(value) & mask;
    let sub = if tmp2 == rounding { 1 } else { 0 };
    result.wrapping_sub(sub)
}
