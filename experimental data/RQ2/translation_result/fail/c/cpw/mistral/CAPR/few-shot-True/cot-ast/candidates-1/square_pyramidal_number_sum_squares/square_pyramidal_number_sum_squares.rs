

use std::ops::Add;
use std::num::Wrapping;

fn square_pylamidal_number_sum_squares(s: i32) -> i32 {
    let mut sum = Wrapping(0);
    let mut n = 1;

    while sum.0 < s as u32 {
        sum += Wrapping(n * n);
        n += 1;
    }

    if sum.0 == s as u32 {
        (n as i32) - 1
    } else {
        -1
    }
}

