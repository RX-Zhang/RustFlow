
fn number_is_divisible_by_29_or_not(n: i64) -> bool {
    let n = n;
    let mut temp = n;

    while temp / 100 != 0 {
        let last_digit = temp % 10;
        temp /= 10;
        temp = temp.wrapping_add(last_digit * 3);
    }
    
    n % 29 == 0
}
