
use std::num::Wrapping;

fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    let opnum = Wrapping(opnum);
    let eighteen = Wrapping(18u32);
    let six = Wrapping(6u32);
    let two = Wrapping(2u32);
    let hundred = Wrapping(0x100u32);

    (opnum % eighteen + two * ((opnum % eighteen) / six) + hundred * (opnum / eighteen)).0
}
