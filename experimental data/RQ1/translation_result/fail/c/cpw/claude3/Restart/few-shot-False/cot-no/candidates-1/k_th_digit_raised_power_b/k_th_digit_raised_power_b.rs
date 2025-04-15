
fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let mut p = (a as f64).powf(b as f64) as i32;
    let mut count = 0;
    
    while p > 0 && count < k {
        let rem = p % 10;
        count += 1;
        if count == k {
            return rem;
        }
        p = p / 10;
    }
    0
}
