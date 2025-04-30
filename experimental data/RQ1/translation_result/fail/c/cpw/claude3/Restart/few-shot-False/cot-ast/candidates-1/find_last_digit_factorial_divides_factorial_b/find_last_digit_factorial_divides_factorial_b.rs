
fn find_last_digit_factorial_divides_factorial_b(a: i64, b: i64) -> i32 {
    let mut variable = 1;
    if a == b {
        1
    } else if (b - a) >= 5 {
        0
    } else {
        for i in (a + 1)..=b {
            variable = (variable * (i % 10).abs() as i32).wrapping_rem(10);
        }
        variable.wrapping_rem(10)
    }
}
