
fn check_if_a_number_is_jumbled_or_not(num: i32) -> i32 {
    if num / 10 == 0 {
        return 1;
    }
    let mut num = num;
    while num != 0 {
        if num / 10 == 0 {
            return 1;
        }
        let digit1 = num % 10;
        let digit2 = (num / 10) % 10;
        if (digit2 - digit1).abs() > 1 {
            return 0;
        }
        num /= 10;
    }
    1
}
