
fn k_th_digit_raised_power_b(a: i32, b: i32, k: usize) -> i32 {
    let p = a.pow(b as u32);
    let mut num = p;
    let mut count = 0;
    while num > 0 && count < k {
        let rem = num % 10;
        count += 1;
        if count == k {
            return rem;
        }
        num /= 10;
    }
    0
}
