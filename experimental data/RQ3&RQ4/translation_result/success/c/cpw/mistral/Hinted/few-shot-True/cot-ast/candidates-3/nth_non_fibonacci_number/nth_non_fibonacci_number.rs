

use std::i32;
use std::ops::{Add, Sub};

fn nth_non_fibonacci_number(mut n: i32) -> i32 {
let mut prev_prev = 1;
let mut prev = 2;
let mut curr = 3;

while n > 0 {
let temp = (curr as i64).saturating_sub((prev as i64).saturating_add(1)).wrapping_add(1) as i32;
curr = (prev_prev as i64).saturating_add((prev as i64).saturating_add(temp as i64)) as i32;
prev_prev = prev;
prev = curr.saturating_sub(temp);
n = n.saturating_sub(1);
}

prev + n
}

