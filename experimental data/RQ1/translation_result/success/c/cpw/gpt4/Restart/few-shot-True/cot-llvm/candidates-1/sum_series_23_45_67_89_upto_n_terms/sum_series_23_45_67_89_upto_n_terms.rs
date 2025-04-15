
fn sum_series_23_45_67_89_upto_n_terms(n: i32) -> f64 {
    let mut i = 1i32;
    let mut res = 0.0f64;
    let mut sign = true;
    let mut n = n;

    while n > 0 {
        n = n.wrapping_sub(1);
        i = i.wrapping_add(1);
        if sign {
            sign = !sign;
            res += i as f64 / i.wrapping_add(1) as f64;
        } else {
            sign = !sign;
            res -= i as f64 / i.wrapping_add(1) as f64;
        }
    }
    res as f64
}
