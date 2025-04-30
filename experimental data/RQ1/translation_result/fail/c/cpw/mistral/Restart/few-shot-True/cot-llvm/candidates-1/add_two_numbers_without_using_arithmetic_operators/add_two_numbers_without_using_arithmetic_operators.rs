

use std::i32;

fn add_two_numbers_without_using_arithmetic_operators(mut x: i32, mut y: i32) -> i32 {
    while y != 0 {
        let carry = ((x & y) as u32).wrapping_shl(1) as i32;
        x = (x as u32).wrapping_add(y as u32) as i32;
        y = carry;
    }
    x
}

