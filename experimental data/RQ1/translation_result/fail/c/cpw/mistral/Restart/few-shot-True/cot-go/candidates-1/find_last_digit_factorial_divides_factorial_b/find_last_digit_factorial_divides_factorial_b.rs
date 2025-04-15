
use std::num::Wrapping;

fn find_last_digit_factorial_divides_factorial_b(a: i64, b: i64) -> i32 {
    if a == b {
        1
    } else if (b - a) >= 5 {
        0
    } else {
        let mut variable = Wrapping(1);
        for i in (a + 1)..=b {
            variable = Wrapping((variable.0 * (i % 10)) % 10);
        }
        (variable.0 % 10) as i32
    }
}
