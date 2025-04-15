
use std::f64;

fn count_digits_factorial_set_1(n: i32) -> i32 {
    if n < 0 {
        0
    } else if n <= 1 {
        1
    } else {
        let digits: f64 = (2..=n).map(|i| f64::log10(i as f64)).sum();
        digits.floor() as i32 + 1
    }
}
