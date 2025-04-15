fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    (chnum % 9).wrapping_add((0x100u32).wrapping_mul(chnum / 9))
}
