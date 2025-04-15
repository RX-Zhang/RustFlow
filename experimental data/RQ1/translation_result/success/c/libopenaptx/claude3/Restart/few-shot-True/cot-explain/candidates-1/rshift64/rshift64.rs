
use std::num::Wrapping;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = Wrapping(1i64 << (shift.wrapping_sub(1) % 64));
    let mask = Wrapping((1i64 << (shift.wrapping_add(1) % 64)).wrapping_sub(1));
    let result = Wrapping(value).0.wrapping_add(rounding.0);
    (result >> (shift % 64)).wrapping_sub((Wrapping(value) & mask == rounding) as i64)
}
