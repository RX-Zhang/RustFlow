
use std::i32;
use std::ops::Shr;

fn square_of_an_integer(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut i: i32 = 1;
    let mut result: i32 = 1;

    while result <= x {
        i += 1;
        result = ((i << 1) as u32).wrapping_shr(1) as i32;
    }

    i - 1
}
