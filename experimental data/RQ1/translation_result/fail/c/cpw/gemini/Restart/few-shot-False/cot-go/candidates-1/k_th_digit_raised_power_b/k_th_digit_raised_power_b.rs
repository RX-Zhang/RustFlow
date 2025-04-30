
fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let mut p = if b >= 0 {
        a.wrapping_pow(b as u32)
    } else {
        0
    };
    let mut count = 0;
    while p > 0 && count < k {
        let rem = p % 10;
        count += 1;
        if count == k {
            return rem;
        }
        p /= 10;
    }
    0
}
