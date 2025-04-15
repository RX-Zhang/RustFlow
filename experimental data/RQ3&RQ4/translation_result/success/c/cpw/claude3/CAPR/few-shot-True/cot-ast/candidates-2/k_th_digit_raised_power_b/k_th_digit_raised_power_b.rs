
fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let p = (a as f64).powf(b as f64);
    
    if !p.is_finite() || p >= (i32::MAX as f64) {
        return 0;
    }
    
    let mut p_int = p as i32;
    let mut count = 0;
    
    while p_int > 0 && count < k {
        let rem = p_int % 10;
        count += 1;
        if count == k {
            return rem;
        }
        p_int /= 10;
    }
    
    0
}
