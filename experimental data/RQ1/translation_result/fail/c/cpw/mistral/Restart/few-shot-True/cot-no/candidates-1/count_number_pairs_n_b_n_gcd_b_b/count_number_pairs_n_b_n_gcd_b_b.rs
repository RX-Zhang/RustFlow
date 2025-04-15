
pub fn count_number_pairs_n_b_n_gcd_b_b(n: i32) -> i32 {
    let mut k = n;
    let mut imin = 1;
    let mut ans = 0;
    while imin <= n {
        if k % imin == 0 {
            let gcd_val = gcd(imin, k / imin);
            ans += gcd_val;
        }
        imin += 1;
    }
    ans
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}
