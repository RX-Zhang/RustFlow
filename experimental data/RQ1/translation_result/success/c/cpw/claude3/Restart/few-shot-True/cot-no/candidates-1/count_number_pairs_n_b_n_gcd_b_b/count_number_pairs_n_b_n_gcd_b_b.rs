
fn count_number_pairs_n_b_n_gcd_b_b(n: i32) -> i32 {
    let mut k = n;
    let mut imin = 1;
    let mut ans: i32 = 0;

    while imin <= n {
        let imax = n / k;
        ans = ans.wrapping_add(k.wrapping_mul(imax.wrapping_sub(imin).wrapping_add(1)));
        imin = imax.wrapping_add(1);
        k = n / imin;
    }

    ans
}
