
use std::num::Wrapping;

fn program_check_plus_perfect_number(x: i32) -> bool {
    let temp = x;
    let mut n = 0;
    let mut x_copy = x.abs();

    while x_copy != 0 {
        x_copy /= 10;
        n += 1;
    }

    x_copy = temp.abs();
    let mut sum = Wrapping(0i32);

    while x_copy != 0 {
        let digit = x_copy % 10;
        sum += Wrapping(digit.pow(n as u32));
        x_copy /= 10;
    }

    sum.0 == temp.abs()
}
