
fn painting_fence_algorithm(n: i32, k: i32) -> i64 {
    let mut total: i64 = k as i64;
    let mod_val: i64 = 1000000007;
    let mut same: i64 = 0;
    let mut diff: i64 = k as i64;

    for _ in 2..=n {
        same = diff;
        diff = (total * (k as i64 - 1)).wrapping_rem(mod_val);
        total = (same + diff).wrapping_rem(mod_val);
    }
    total
}
