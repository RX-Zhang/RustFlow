
use std::num::Wrapping;

fn find_first_natural_number_whose_factorial_divisible_x(x: i32) -> i32 {
    let mut i = 1;
    let mut fact = Wrapping(1);

    for _ in 1..x {
        fact *= Wrapping(i);
        if fact % Wrapping(x) == Wrapping(0) {
            break;
        }
        i += 1;
    }

    i
}
