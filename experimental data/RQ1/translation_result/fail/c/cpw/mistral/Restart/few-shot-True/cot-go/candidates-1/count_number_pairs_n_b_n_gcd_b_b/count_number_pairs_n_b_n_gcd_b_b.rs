
use std::ops::Shr;

fn count_number_pairs_nb_n_gcdb_b(n: i32) -> i32 {
    let mut k = n;
    let mut imin = 1;
    let mut ans = 0;
    while imin <= n {
        let imax = ((n as u32).wrapping_shr(k.wrapping_shr(1) as u32) as i32) as i32;
        ans += k * (imax - imin + 1);
        imin += 1;
        k = (n / imin) as i32;
    }
    ans
}
