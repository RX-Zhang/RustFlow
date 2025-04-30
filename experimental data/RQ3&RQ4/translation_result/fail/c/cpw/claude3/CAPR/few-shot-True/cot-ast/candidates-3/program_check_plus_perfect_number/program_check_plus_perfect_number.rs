
use std::num::Wrapping;

fn program_check_plus_perfect_number(x: i32) -> bool {
    let temp = x;
    let mut n = 0;
    let mut x_mutable = x;

    while x_mutable != 0 {
        x_mutable /= 10;
        n += 1;
    }

    x_mutable = temp;
    let mut sum = Wrapping(0i64);

    while x_mutable != 0 {
        let digit = x_mutable % 10;
        sum += Wrapping(i64::from(digit).wrapping_pow(n as u32));
        x_mutable /= 10;
    }

    sum.0 == i64::from(temp)
}
