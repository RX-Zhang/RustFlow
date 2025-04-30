

use std::u32;

fn count_set_bits_in_an_integer(n: u32) -> u32 {
    let mut n_local = n;
    let mut count = 0;
    while n_local > 0 {
        count += n_local as u32 & 1;
        n_local = n_local.wrapping_shr(1);
    }
    count
}

