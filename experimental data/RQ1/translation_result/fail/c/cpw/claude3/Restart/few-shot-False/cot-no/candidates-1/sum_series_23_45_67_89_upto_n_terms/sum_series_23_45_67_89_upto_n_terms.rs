
fn sum_series_23_45_67_89_upto_n_terms(n: i32) -> f32 {
    let mut i: i32 = 1;
    let mut res: f64 = 0.0;
    let mut sign = true;
    let mut remaining_terms = n;

    while remaining_terms > 0 {
        remaining_terms = remaining_terms.wrapping_sub(1);
        if sign {
            sign = !sign;
            i = i.wrapping_add(1);
            let numerator = i;
            i = i.wrapping_add(1);
            res += numerator as f64 / i as f64;
        } else {
            sign = !sign;
            i = i.wrapping_add(1);
            let numerator = i;
            i = i.wrapping_add(1);
            res -= numerator as f64 / i as f64;
        }
    }

    res as f32
}
