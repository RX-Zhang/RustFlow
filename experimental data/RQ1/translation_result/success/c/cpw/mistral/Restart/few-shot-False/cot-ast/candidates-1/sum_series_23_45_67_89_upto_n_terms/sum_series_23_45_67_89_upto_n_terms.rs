fn sum_series_23_45_67_89_upto_n_terms(n: i32) -> f32 {
    let mut i: i32 = 1;
    let mut res: f64 = 0.0;
    let mut sign: bool = true;
    let mut n_remaining = n;
    while n_remaining > 0 {
        n_remaining = n_remaining.wrapping_sub(1);
        if sign {
            sign = !sign;
            i = i.wrapping_add(1);
            res = res + (i.wrapping_add(0) as f64 / i.wrapping_add(1) as f64);
            i = i.wrapping_add(1);
        } else {
            sign = !sign;
            i = i.wrapping_add(1);
            res = res - (i.wrapping_add(0) as f64 / i.wrapping_add(1) as f64);
            i = i.wrapping_add(1);
        }
    }
    res as f32
}