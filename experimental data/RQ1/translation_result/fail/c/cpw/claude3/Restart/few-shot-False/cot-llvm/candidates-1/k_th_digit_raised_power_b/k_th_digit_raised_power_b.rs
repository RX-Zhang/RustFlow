
fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let mut p = (a as i64).wrapping_pow(b as u32);
    let mut count = 0;
    while p > 0 && count < k {
        let rem = p % 10;
        count += 1;
        if count == k {
            return rem as i32;
        }
        p /= 10;
    }
    0
}
