
use std::u32;

fn painting_fence_algorithm(n: u32, k: u32) -> u32 {
    let modulo = 1_000_000_007;
    let mut total = k as u32;
    let mut same = 0;
    let mut diff = k as u32;

    for i in 2..=n {
        same = diff;
        diff = (total.wrapping_mul(k - 1)).rem_euclid(modulo);
        total = (same.wrapping_add(diff)).rem_euclid(modulo);
    }

    total
}
