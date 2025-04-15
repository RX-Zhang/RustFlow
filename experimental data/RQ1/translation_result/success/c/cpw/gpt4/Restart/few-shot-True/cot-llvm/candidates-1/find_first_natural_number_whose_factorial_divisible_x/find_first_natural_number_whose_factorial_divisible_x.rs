
use std::num::Wrapping;

fn find_first_natural_number_whose_factorial_divisible_x(x: i32) -> i32 {
    let mut i = 1;
    let mut fact = Wrapping(1);

    while i < x {
        fact = fact * Wrapping(i);
        if fact.0 % x == 0 {
            break;
        }
        i += 1;
    }
    i
}
