
fn painting_fence_algorithm(n: i32, k: i32) -> i64 {
    let mut total: i64 = k as i64;
    let mod_val: i64 = 1_000_000_007;
    let mut same: i64 = 0;
    let mut diff: i64 = k as i64;

    for _ in 2..=n {
        same = diff;
        diff = total.wrapping_mul((k - 1) as i64).wrapping_rem(mod_val);
        total = (same.wrapping_add(diff)).wrapping_rem(mod_val);
    }

    total
}
