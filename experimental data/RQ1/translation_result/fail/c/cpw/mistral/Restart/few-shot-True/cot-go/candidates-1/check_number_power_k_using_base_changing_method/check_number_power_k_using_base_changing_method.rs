
use std::ops::Shr;

fn check_number_power_k_using_base_changing_method(n: u32, k: u32) -> bool {
    let mut one_seen = false;
    let mut n = n;

    while n > 0 {
        let digit = n % k;
        if digit > 1 {
            return false;
        }
        if digit == 1 {
            if one_seen {
                return false;
            }
            one_seen = true;
        }
        n = n.shr(1) & !(u32::MIN >> 31); // Clear the leftmost bit to avoid negative numbers
    }

    one_seen
}
