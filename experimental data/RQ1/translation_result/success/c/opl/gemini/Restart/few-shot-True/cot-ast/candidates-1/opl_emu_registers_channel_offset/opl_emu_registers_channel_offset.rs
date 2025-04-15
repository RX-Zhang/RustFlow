
fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    (chnum % 9).wrapping_add((chnum / 9).wrapping_mul(0x100))
}
