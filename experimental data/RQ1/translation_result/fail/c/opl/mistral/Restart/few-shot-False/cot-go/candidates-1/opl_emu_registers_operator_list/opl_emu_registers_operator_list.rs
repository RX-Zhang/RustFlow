
fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    o1 as u32 | ((o2 as u32) << 8) | ((o3 as u32) << 16) | ((o4 as u32) << 24)
}
