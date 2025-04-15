
use std::f64;

fn count_digits_factorial_set_1(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    if n <= 1 {
        return 1;
    }
    let mut digits = 0.0;
    for i in 2..=n {
        digits += (i as f64).log10();
    }
    (digits.floor() as i32) + 1
}
