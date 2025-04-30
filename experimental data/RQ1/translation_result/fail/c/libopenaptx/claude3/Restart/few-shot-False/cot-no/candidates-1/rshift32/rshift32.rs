
use std::num::Wrapping;

#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = Wrapping(1i32 << (shift.wrapping_sub(1) % 32));
    let mask = Wrapping((1i32 << (shift.wrapping_add(1) % 32)).wrapping_sub(1));
    let result = Wrapping(value).0.wrapping_add(rounding.0);
    (result >> (shift % 32)).wrapping_sub(
        if (Wrapping(value) & mask).0 == rounding.0 { 1 } else { 0 }
    )
}
