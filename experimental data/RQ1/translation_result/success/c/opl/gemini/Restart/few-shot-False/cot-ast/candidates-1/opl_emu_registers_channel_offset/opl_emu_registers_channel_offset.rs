
use std::num::Wrapping;

fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    let wrapped_chnum = Wrapping(chnum);
    (wrapped_chnum % Wrapping(9) + Wrapping(0x100) * (wrapped_chnum / Wrapping(9))).0
}
