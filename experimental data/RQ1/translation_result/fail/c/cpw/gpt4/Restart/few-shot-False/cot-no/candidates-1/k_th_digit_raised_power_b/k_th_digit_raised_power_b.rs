

fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let p = (a as f64).powi(b) as i32;
    let mut count = 0;
    let mut p = p;

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
