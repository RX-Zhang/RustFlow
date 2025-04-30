
use std::num::Wrapping;

fn count_factorial_numbers_in_a_given_range(low: i32, high: i32) -> i32 {
    let mut fact = Wrapping(1);
    let mut x = Wrapping(1);
    
    while fact < Wrapping(low) {
        fact = fact * x;
        x = x + Wrapping(1);
    }
    
    let mut res = 0;
    while fact <= Wrapping(high) {
        res += 1;
        fact = fact * x;
        x = x + Wrapping(1);
    }
    
    res
}
