use std::num::Wrapping;
use std::ops::Shr;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = Wrapping((1u32 << (length as u32 % 32)) - 1);
    (Wrapping(value).shr(start as usize % 32) & mask).0
}
