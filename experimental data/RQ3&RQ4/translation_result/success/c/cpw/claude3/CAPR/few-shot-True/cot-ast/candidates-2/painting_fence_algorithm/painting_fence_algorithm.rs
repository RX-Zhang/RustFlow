fn painting_fence_algorithm(n: i32, k: i32) -> i64 {
    let mut total: i64 = k as i64;
    let mod_val: i64 = 1_000_000_007;
    let mut same: i64 = 0;
    let mut diff: i64 = k as i64;

    let mut i: i32 = 2;
    while i <= n {
        let prev_same = same;
        same = diff;
        diff = total.wrapping_mul((k as i64).wrapping_sub(1)).wrapping_rem(mod_val);
        total = (prev_same.wrapping_add(diff)).wrapping_rem(mod_val);
        i = i.wrapping_add(1);
    }

    total
}