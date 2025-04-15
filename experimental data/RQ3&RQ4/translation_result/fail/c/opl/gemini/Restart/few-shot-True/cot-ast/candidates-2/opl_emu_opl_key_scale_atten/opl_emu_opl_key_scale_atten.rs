fn opl_emu_opl_key_scale_atten(block: u32, fnum_4msb: u32) -> u32 {
    let fnum_to_atten: [u8; 16] = [0, 24, 32, 37, 40, 43, 45, 47, 48, 50, 51, 52, 53, 54, 55, 56];
    let result = fnum_to_atten[fnum_4msb as usize] as i32 - 8 * ((block ^ 7) as i32)
    result.max(0) as u32
}
