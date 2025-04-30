
fn add_two_numbers_without_using_arithmetic_operators(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let carry = x & y;
        x = x ^ y;
        y = carry.wrapping_shl(1);
    }
    x
}
