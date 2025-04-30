
fn sum_series_23_45_67_89_upto_n_terms(mut n: i32) -> f64 {
    let mut i = 1;
    let mut res = 0.0;
    let mut sign = true;
    while n > 0 {
        n -= 1;
        if sign {
            sign = false;
            res += (i + 1) as f64 / (i + 1) as f64;
        } else {
            sign = true;
            res -= (i + 1) as f64 / (i + 1) as f64;
        }
        i += 1;
    }
    res
}
