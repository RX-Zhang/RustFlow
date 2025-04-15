
use std::ops::{BitAnd, Shr};

fn parity(mut n: u32) -> bool {
let mut parity = false;
while n > 0 {
parity = !parity;
let tmp = n.wrapping_sub(1);
n = n & tmp;
}
parity
}
