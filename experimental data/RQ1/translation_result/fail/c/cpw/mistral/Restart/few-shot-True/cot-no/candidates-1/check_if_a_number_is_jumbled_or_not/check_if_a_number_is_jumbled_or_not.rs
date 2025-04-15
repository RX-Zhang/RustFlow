
use std::cmp::Ordering;
use std::i32;

fn check_if_a_number_is_jumbled_or_not(num: i32) -> bool {
    if num.wrapping_div(10) == 0 {
        return true;
    }
    let mut num = num;
    while num != 0 {
        if num.wrapping_div(10) == 0 {
            return true;
        }
        let digit1 = num % 10;
        let digit2 = (num.wrapping_div(10)) % 10;
        let diff = ((digit2 as i32) - (digit1 as i32)).abs();
        if diff > 1 {
            return false;
        }
        num = num.wrapping_div(10);
    }
    true
}
