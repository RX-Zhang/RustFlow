
fn add_two_numbers_without_using_arithmetic_operators(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    
    while y != 0 {
        let carry = x & y;
        x = x ^ y;
        y = (carry as i32).wrapping_shl(1);
    }
    
    x
}
