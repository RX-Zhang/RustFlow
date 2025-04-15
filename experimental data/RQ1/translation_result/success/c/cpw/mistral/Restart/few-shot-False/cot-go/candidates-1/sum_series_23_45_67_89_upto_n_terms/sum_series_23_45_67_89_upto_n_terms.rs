
fn sum_series_23_45_67_89_upto_n_terms(n: i32) -> f32 {
    let mut i: i32 = 1;
    let mut res = 0.0;
    let mut sign = true;
    let mut remaining_terms = n;

    while remaining_terms > 0 {
        remaining_terms -= 1;
        if sign {
            sign = !sign;
            i = i.wrapping_add(1);
            let numerator = i as f64;
            i = i.wrapping_add(1);
            let denominator = i as f64;
            res += numerator / denominator;
        } else {
            sign = !sign;
            i = i.wrapping_add(1);
            let numerator = i as f64;
            i = i.wrapping_add(1);
            let denominator = i as f64;
            res -= numerator / denominator;
        }
    }

    res as f32
}
