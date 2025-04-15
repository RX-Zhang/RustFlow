

use std::ops::{BitAnd, Shr};

fn count_set_bits_in_an_integer(n: u32) -> u32 {
let mut n = n;
let mut count = 0;
while n > 0 {
count += n & 1;
n = n.wrapping_shr(1);
}
count
}

