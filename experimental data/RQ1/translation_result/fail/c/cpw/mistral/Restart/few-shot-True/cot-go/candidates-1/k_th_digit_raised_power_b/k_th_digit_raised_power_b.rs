
fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let p = if b >= 0 {
        a.wrapping_pow(b as u32)
    } else if a == -1 {
        if b % 2 == 0 { 1 } else { -1 }
    } else if a == 1 {
        1
    } else {
        0
    };

    let mut count = 0;
    let mut p_mut = p.abs();

    while p_mut > 0 && count < k {
        let rem = p_mut % 10;
        count += 1;
        if count == k {
            return rem;
        }
        p_mut /= 10;
    }
    0
}
