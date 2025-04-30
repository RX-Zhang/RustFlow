
fn painting_fence_algorithm(n: i32, k: i32) -> i32 {
    let mut total = k as i64;
    let mod_val = 1_000_000_007;
    let mut same = 0_i64;
    let mut diff = k as i64;

    for _ in 2..=n {
        same = diff;
        diff = total.wrapping_mul((k - 1) as i64) % mod_val;
        total = (same + diff) % mod_val;
    }

    total as i32
}
