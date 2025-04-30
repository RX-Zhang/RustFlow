
use std::num::Wrapping;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = Wrapping(1i64).0.wrapping_shl(shift.wrapping_sub(1) % 64);
    let mask = (Wrapping(1i64).0.wrapping_shl(shift.wrapping_add(1) % 64)).wrapping_sub(1);
    let result = (Wrapping(value).0.wrapping_add(rounding)).wrapping_shr(shift % 64);
    result.wrapping_sub((value & mask == rounding) as i64)
}
