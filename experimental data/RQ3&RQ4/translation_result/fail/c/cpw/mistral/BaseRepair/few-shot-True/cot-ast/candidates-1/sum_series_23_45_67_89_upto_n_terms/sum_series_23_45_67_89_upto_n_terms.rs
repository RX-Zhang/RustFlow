
use std::ops::AddAssign;

fn sum_series_23__67_8_upt_nterms(n: u32) -> f64 {
    let mut i: u32 = 1;
    let mut res: f64 = 0.0;
    let mut sign = true;
    let mut n_inner = n;

    while n_inner > 0 {
        n_inner -= 1;
        sign = !sign;
        i = i.wrapping_add(1);
        if sign {
            res += (i as f64) / ((i as u32).wrapping_add(1) as f64);
        } else {
            res -= (i as f64) / ((i as u32).wrapping_add(1) as f64);
        }
    }

    res
}
