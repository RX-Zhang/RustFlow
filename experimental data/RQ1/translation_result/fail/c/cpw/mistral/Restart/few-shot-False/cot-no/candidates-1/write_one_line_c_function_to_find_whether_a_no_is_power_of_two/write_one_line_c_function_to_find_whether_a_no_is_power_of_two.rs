
use std::i32;

fn is_power_of_two(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    let mut n_shr = n;
    while n_shr != 1 {
        if n_shr % 2 != 0 {
            return false;
        }
        n_shr = n_shr.wrapping_shr(1);
    }
    return true;
}
