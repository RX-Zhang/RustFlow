
use std::i32;

fn check_if_a_number_is_j_or_not(num: i32) -> bool {
    if num / 10 == 0 {
        return true;
    }
    let mut num = num;
    while num != 0 {
        if num / 10 == 0 {
            return true;
        }
        let digit1 = num % 10;
        let digit2 = (num / 10) % 10;
        if (digit2 as i32 - digit1 as i32).abs() > 1 {
            return false;
        }
        num = num / 10;
    }
    true
}
