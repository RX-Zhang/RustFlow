
use std::num::Wrapping;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 32u32.wrapping_sub(bits);
    let shifted = Wrapping(val as u32).0.wrapping_shl(shift);
    (shifted as i32).wrapping_shr(shift)
}
