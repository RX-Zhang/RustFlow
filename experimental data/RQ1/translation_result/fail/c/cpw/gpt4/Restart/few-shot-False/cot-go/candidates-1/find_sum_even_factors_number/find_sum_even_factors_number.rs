
fn find_sum_even_factors_number(mut n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }

    let mut res = 1_i32;
    let mut i = 2_i32;

    while i <= f64::from(n).sqrt() as i32 {
        let mut count = 0_i32;
        let mut curr_sum = 1_i32;
        let mut curr_term = 1_i32;

        while n % i == 0 {
            count += 1;
            n /= i;
            if i == 2 && count == 1 {
                curr_sum = 0;
            }
            curr_term = curr_term.wrapping_mul(i);
            curr_sum = curr_sum.wrapping_add(curr_term);
        }
        res = res.wrapping_mul(curr_sum);
        i += 1;
    }

    if n >= 2 {
        res = res.wrapping_mul(1_i32.wrapping_add(n));
    }

    res
}
