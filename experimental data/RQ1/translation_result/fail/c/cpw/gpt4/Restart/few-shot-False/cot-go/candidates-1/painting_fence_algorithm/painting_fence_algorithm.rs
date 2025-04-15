
fn painting_fence_algorithm(n: i32, k: i32) -> i64 {
    let mut total = k as i64;
    let mod_val = 1000000007;
    let mut same = 0;
    let mut diff = k as i64;

    for i in 2..=n {
        same = diff;
        diff = (total.wrapping_mul((k - 1) as i64)) % mod_val;
        total = (same + diff) % mod_val;
    }
    
    total
}
