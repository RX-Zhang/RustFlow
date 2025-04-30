
use std::cmp;

fn opl_max(a: u32, b: u32) -> u32 {
    cmp::max(a, b)
}

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let fnum_to_atten: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];
    let result = i32::from(fnum_to_atten[fnum_4msb as usize])
        .wrapping_sub(8_i32.wrapping_mul((block ^ 7) as i32));
    opl_max(0, result as u32)
}
