
use std::i32;

fn wrapping_add(a: i32, b: i32) -> i32 {
    i32::wrapping_add(a, b)
}

fn wrapping_shr(a: i32, b: i32) -> i32 {
    let shifted = ((a as u32) >> (b & 31)) as i32;
    shifted
}
