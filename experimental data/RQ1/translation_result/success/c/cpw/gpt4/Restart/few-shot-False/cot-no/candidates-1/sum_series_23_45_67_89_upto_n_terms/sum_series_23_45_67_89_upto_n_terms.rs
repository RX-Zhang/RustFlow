
use std::boxed::Box;

fn sum_series_23_45_67_89_upto_n_terms(n: i32) -> f64 {
    let mut i: i32 = 1;
    let mut res: f64 = 0.0;
    let mut sign: bool = true;
    let boxed_n = Box::new(n);
    let mut n = *boxed_n;

    while n > 0 {
        n -= 1;
        if sign {
            sign = !sign;
            res += (i.wrapping_add(1) as f64) / ((i.wrapping_add(2)) as f64);
            i = i.wrapping_add(2);
        } else {
            sign = !sign;
            res -= (i.wrapping_add(1) as f64) / ((i.wrapping_add(2)) as f64);
            i = i.wrapping_add(2);
        }
    }

    res
}
