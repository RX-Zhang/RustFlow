use std::num::Wrapping;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 32u32.wrapping_sub(bits);
    let mask = Wrapping(-1i32).0.wrapping_shl(shift);
    let extended = Wrapping(val).0.wrapping_shl(shift);
    extended.wrapping_shr(shift)
}
