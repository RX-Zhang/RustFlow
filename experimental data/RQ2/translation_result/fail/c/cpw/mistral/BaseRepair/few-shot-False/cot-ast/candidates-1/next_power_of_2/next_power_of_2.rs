
fn rotate_left(val: i32, shift_amt: u8) -> i32 {
    let rotated_bit_pattern = ((val as u32).wrapping_shl(shift_amt as u32)) | ((val as u32).wrapping_shr(32 - (shift_amt as u32)));
    rotated_bit_pattern as i32
}
