
fn count_number_pairs_n_b_n_gcd_b_b(n: i32) -> i32 {
    let mut k = n;
    let mut imin = 1;
    let mut ans: i32 = 0; // Specify type for ans

    while imin <= n {
        let imax = n / k;
        ans = ans.wrapping_add(k.wrapping_mul(imax - imin + 1));
        imin = imax + 1;
        k = n / imin;
    }
    
    ans
}
