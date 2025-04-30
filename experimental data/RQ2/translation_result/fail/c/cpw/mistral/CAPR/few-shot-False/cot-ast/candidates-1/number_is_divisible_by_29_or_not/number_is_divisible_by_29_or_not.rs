
fn number_is_divisible_by_29_or_not(mut n: i64) -> bool {
    while n / 100 != 0 {
        let last_digit = n.wrapping_rem(10);
        n = n.wrapping_div(10);
        n = n.wrapping_add(last_digit.wrapping_mul(3));
    }
    n.wrapping_rem(29) == 0
}
