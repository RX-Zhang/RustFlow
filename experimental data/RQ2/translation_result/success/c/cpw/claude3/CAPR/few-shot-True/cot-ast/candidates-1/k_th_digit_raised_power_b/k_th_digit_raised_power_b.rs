fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let p = a.pow(b as u32);
    let mut count: i32 = 0;
    let mut p_mut = p;

    while p_mut > 0 && count < k {
        let rem = p_mut.wrapping_rem(10);
        count = count.wrapping_add(1);
        if count == k {
            return rem;
        }
        p_mut = p_mut.wrapping_div(10);
    }

    0
}