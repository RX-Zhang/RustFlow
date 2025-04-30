fn check_if_a_number_is_jumbled_or_not(num: i32) -> i32 {
    let mut num = num;
    if num / 10 == 0 {
        return 1;
    }
    while num != 0 {
        if num / 10 == 0 {
            return 1;
        }
        let digit1 = (num % 10).wrapping_abs();
        let digit2 = ((num / 10) % 10).wrapping_abs();
        if (digit2.wrapping_sub(digit1)).wrapping_abs() > 1 {
            return 0;
        }
        num = num / 10;
    }
    1
}