
use std::u32;
use std::i32;

fn smallest_power_of_2_greater_than_or__to_n(n: i32) -> i32 {
    let mut n = n as u32;
    n -= 1;
    n = (n >> 1).wrapping_add(n) & 0xFFFF_FFFF;
    n = (n >> 2).wrapping_add(n) & 0xFFFF_FFFF;
    n = (n >> 4).wrapping_add(n) & 0xFFFF_FFFF;
    n = (n >> 8).wrapping_add(n) & 0xFFFF_FFFF;
    n = (n >> 16).wrapping_add(n) & 0xFFFF_FFFF;
    (n.wrapping_add(1) as i32)
}
