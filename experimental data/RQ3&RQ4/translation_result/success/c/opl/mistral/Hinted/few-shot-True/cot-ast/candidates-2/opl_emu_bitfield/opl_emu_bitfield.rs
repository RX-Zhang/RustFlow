fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let shift_amount = start % 32;
    let mask = (1u32.wrapping_shl(length as u32).wrapping_sub(1)) as u32;
    (value.wrapping_shr(shift_amount as u32)) & mask
}
