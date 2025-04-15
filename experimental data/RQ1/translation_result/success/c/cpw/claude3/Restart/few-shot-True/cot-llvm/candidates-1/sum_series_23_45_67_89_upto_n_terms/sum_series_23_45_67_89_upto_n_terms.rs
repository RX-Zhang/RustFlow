
fn sum_series_23_45_67_89_upto_n_terms(mut n: i32) -> f32 {
    let mut i: i32 = 1;
    let mut res: f64 = 0.0;
    let mut sign = true;

    while n > 0 {
        n = n.wrapping_sub(1);
        if sign {
            sign = !sign;
            i = i.wrapping_add(1);
            let numerator = i as f64;
            i = i.wrapping_add(1);
            res += numerator / i as f64;
        } else {
            sign = !sign;
            i = i.wrapping_add(1);
            let numerator = i as f64;
            i = i.wrapping_add(1);
            res -= numerator / i as f64;
        }
    }

    res as f32
}
