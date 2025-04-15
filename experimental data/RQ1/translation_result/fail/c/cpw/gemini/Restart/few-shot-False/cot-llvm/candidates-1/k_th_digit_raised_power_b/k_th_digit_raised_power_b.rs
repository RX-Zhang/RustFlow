
use std::i32;

fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let p = a.pow(b as u32) as i64; // Use i64 to avoid overflow
    let mut count = 0;
    let mut p_local = p;
    while p_local > 0 && count < k {
        let rem = (p_local % 10) as i32;
        count += 1;
        if count == k {
            return rem;
        }
        p_local /= 10;
    }
    0
}
