

fn number_is_divisible_by_29_or_not(n: i64) -> bool {
    let mut num = n;
    while num / 100 > 0 {
        let last_digit = (num % 10) as i64;
        num /= 10;
        num += last_digit * 3;
    }
    (num % 29) == 0
}
