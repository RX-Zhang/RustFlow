
fn find_last_digit_factorial_divides_factorial_b(a: i64, b: i64) -> i32 {
    if a == b {
        return 1;
    } else if (b - a) >= 5 {
        return 0;
    } else {
        let mut variable = 1;
        for i in (a + 1)..=b {
            variable = (variable * (i % 10)) % 10;
        }
        return variable as i32;
    }
}
