
fn number_is_divisible_by_29_or_not(mut n: i64) -> bool {
    while n / 100 != 0 {
        let last_digit = n % 10;
        n /= 10;
        n = n.wrapping_add(last_digit.wrapping_mul(3));
    }
    n % 29 == 0
}
