fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    (opnum.wrapping_rem(18))
        .wrapping_add(2 * (opnum.wrapping_rem(18) / 6))
        .wrapping_add(0x100u32.wrapping_mul(opnum.wrapping_div(18)))
}
