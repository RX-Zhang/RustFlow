
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let shifted = value.wrapping_shr(start as u32);
    let mask = (1u32.wrapping_shl(length as u32)).wrapping_sub(1);
    shifted & mask
}
