
fn rot_left(val: i32, shift_amt: u8) -> i32 {
    let shifted_val: u32 = (val as u32).wrapping_shl((shift_amt as u32) % 32);
    let rotated_val = (shifted_val | (val as u32).wrapping_shr((32 - (shift_amt as u32)) % 32)) as i32;
    rotated_val
}

fn rotate_right(val: i32, shift_amt: u8) -> i32 {
    let shifted_val: u32 = (val as u32).wrapping_shr(shift_amt as u32 % 32);
    let rotated_val = (shifted_val | (val as u32).wrapping_shl((32 - shift_amt as u32) % 32)) as i32;
    rotated_val
}
