use std::num::Wrapping;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = Wrapping(1i64 << (shift.wrapping_sub(1) % 64));
    let mask = Wrapping((1i64 << (shift.wrapping_add(1) % 64)).wrapping_sub(1));
    let result = ((Wrapping(value) + rounding).0 >> (shift % 64));
    let adjustment = ((Wrapping(value) & mask) == rounding) as i64;
    result.wrapping_sub(adjustment)
}
