
fn painting_fence_algorithm(n: i32, k: i32) -> i32 {
    let mut total = k;
    const MOD: i32 = 1000000007;
    let mut same = 0;
    let mut diff = k;
    for i in 2..=n {
        same = diff;
        diff = (total * (k - 1)) % MOD;
        total = (same + diff) % MOD;
    }
    total
}
