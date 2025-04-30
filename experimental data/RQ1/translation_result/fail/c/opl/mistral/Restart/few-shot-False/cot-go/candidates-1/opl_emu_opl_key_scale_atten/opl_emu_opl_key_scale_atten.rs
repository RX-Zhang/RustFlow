
fn opl_max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let fnum_to_atten: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];
    let safe_fnum_4msb = (fnum_4msb.wrapping_shr(6) & 0xF) as usize;
    let safe_block = block & 0x7;
    let result = fnum_to_atten[safe_fnum_4msb] as i32 - 8_i32.wrapping_mul((safe_block ^ 7) as i32);
    opl_max(0, result) as u32
}
