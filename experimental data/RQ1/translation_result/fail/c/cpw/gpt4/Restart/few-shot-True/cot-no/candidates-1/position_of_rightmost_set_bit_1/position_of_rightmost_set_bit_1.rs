
fn position_of_rightmost_set_bit_1(n: i32) -> i32 {
    let mut position = 1;
    let mut m = 1;
    while (n & m) == 0 {
        m = m.wrapping_shl(1);
        position += 1;
    }
    position
}
