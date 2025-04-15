
use std::num::Wrapping;

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    let result = Wrapping(o1 as u32)
        | (Wrapping(o2 as u32) << 8)
        | (Wrapping(o3 as u32) << 16)
        | (Wrapping(o4 as u32) << 24);
    result.0
}
