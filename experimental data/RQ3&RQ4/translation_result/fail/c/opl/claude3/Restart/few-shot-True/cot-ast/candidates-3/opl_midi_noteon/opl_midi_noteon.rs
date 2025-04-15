
use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = (1u32 << (length as u32 % 32)).wrapping_sub(1);
    (value >> (start as u32 % 32)) & mask
}
