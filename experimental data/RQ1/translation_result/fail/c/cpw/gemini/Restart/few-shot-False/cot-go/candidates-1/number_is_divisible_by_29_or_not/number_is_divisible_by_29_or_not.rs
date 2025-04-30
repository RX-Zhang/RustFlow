
fn number_is_divisible_by_29_or_not(n: i64) -> bool {
    let mut n = n;
    while n >= 100 {
        let last_digit = n % 10;
        n /= 10;
        n = n.wrapping_add(last_digit * 3);
    }
    n % 29 == 0
}
