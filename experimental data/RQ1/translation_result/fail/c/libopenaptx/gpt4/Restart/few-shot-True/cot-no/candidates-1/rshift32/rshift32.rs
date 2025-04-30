
fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1 << (shift - 1);
    let mask: i32 = (1 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i32
}
