
fn sum_series_23_45_67_89_upto_n_terms(n: i32) -> f64 {
    let mut i = 1;
    let mut res = 0.0_f64;
    let mut remaining_terms = n;
    let mut sign = true;

    while remaining_terms > 0 {
        remaining_terms -= 1;
        i += 1;

        let next_i = i + 1;

        if sign {
            sign = !sign;
            res += (i as f64) / (next_i as f64);
        } else {
            sign = !sign;
            res -= (i as f64) / (next_i as f64);
        }

        i += 1;
    }

    res
}
