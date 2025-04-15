
#[no_mangle]
pub extern "C" fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let offset = (opnum % 18).wrapping_add(2u32.wrapping_mul((opnum % 18) / 6));
    offset.wrapping_add(0x100u32.wrapping_mul(opnum / 18))
}
