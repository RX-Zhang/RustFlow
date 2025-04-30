
use std::f64;

fn count_digits_ial_set_1(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    let limit = i32::MAX as f64;
    if n <= 1 {
        return 1;
    }
    let mut digits = 0f64;
    let mut i = 2;
    while i <= n && digits <= limit {
        digits += f64::log10(i as f64);
        i += 1;
    }
    if digits > limit {
        return -(n as i32);
    }
    (digits as i32).wrapping_add(1)
}
