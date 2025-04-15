
use std::num::Wrapping;

fn find_whether_a_given_number_is_a_power_of_4_or_not_1(n: u32) -> i32 {
    let mut count = 0;
    let mut n = Wrapping(n);
    
    if n.0 != 0 && (n & (n - Wrapping(1))).0 == 0 {
        while n.0 > 1 {
            n = n >> 1;
            count += 1;
        }
        if count % 2 == 0 { 1 } else { 0 }
    } else {
        0
    }
}
