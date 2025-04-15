
pub fn find_last_digit_factorial_divides_factorial_b(a: i64, b: i64) -> i32 {
    if a == b {
        1
    } else if (b - a) >= 5 {
        0
    } else {
        let mut variable = 1i32;
        for i in (a + 1..=b).rev() {
            variable = (variable.wrapping_mul(i as i32 % 10) % 10) as i32;
        }
        variable
    }
}
