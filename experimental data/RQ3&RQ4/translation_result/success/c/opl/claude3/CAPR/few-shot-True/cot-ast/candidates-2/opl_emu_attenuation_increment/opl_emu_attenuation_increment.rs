use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    if start >= 32 || length <= 0 || start < 0 {
        return 0;
    }
    let mask = if length >= 32 {
        u32::MAX
    } else {
        (1u32 << length as u32).wrapping_sub(1)
    };
    (value >> start as u32) & mask
}

fn opl_emu_attenuation_increment(rate: u32, index: u32) -> u32 {
    static INCREMENT_TABLE: [u32; 64] = [
        0x00000000, 0x00000000, 0x10101010, 0x10101010,
        0x10101010, 0x10101010, 0x11101110, 0x11101110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x10101010, 0x10111010, 0x11101110, 0x11111110,
        0x11111111, 0x21112111, 0x21212121, 0x22212221,
        0x22222222, 0x42224222, 0x42424242, 0x44424442,
        0x44444444, 0x84448444, 0x84848484, 0x88848884,
        0x88888888, 0x88888888, 0x88888888, 0x88888888
    ];
    let table_index = (Wrapping(rate) % Wrapping(64)).0 as usize;
    let shift_amount = (Wrapping(4) * Wrapping(index)).0 % 32;
    opl_emu_bitfield(INCREMENT_TABLE[table_index], shift_amount as i32, 4)
}
