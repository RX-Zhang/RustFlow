
use std::num::Wrapping;

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = Wrapping(1i32 << (shift.wrapping_sub(1) % 32));
    let mask = Wrapping((1i32 << (shift.wrapping_add(1) % 32)).wrapping_sub(1));
    let shifted = Wrapping(value).0.wrapping_add(rounding.0).wrapping_shr(shift % 32);
    shifted.wrapping_sub((((value & mask.0) == rounding.0) as i32).wrapping_mul(1))
}
