
use std::usize;
use std::i32;
use std::ops::Shr;
use std::num::Wrapping;

fn k_th_digit_raised_power_b(a: i32, b: i32, k: usize) -> i32 {
    let p = i32::pow(a, b.try_into().unwrap()); // Converting b to u32
    let mut count = 0;
    let mut p_boxed = Box::new(p as u64);
    while (*p_boxed > 0) && (count < k) {
        let rem = ((*p_boxed % 10) as i32).rem_euclid(10); // Handling the range of %
        count += 1;
        if count == k {
            return rem;
        }
        *p_boxed = Wrapping((*p_boxed as u64).shr(1) as u64).0; // Using wrapping_shr and 'as' for bitwise operations
    }
    0
}
