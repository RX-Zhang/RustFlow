
use std::u32;
use std::num::Wrapping;

fn smallest_power_of_2_greater_than_or_equal_to_n(n: u32) -> u32 {
    if n & (n - 1) == 0 {
        return n;
    }

    let mut count = 0;
    let mut n_wrapped = Wrapping(n);
    while n_wrapped != Wrapping(0) {
        n_wrapped >>= 1;
        count += 1;
    }

    (1u32 << count)
}
