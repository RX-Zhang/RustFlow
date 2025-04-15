
use std::u32;

fn smallest_power_of_2_greater_than_or_equal_to_n_2(n: u32) -> u32 {
    let mut n = n as u32;
    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n += 1;
    (n as u32).saturating_add(1)
}
