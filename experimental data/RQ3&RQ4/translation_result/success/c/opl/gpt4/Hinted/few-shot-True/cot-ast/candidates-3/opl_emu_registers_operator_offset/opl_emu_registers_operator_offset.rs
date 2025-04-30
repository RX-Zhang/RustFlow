
fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let mod_18 = opnum.wrapping_rem(18);
    let div_18 = opnum.wrapping_div(18);
    let div_6 = mod_18.wrapping_div(6);
    
    mod_18.wrapping_add(2u32.wrapping_mul(div_6)).wrapping_add(0x100u32.wrapping_mul(div_18))
}
