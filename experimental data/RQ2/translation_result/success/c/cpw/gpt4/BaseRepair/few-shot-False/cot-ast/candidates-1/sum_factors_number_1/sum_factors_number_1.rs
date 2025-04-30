fn sum_factors_number_1(n: i32) -> i32 {
    let mut res: i32 = 1;
    let mut n_remaining = n;
    let sqrt_n = (n_remaining as f64).sqrt() as i32;

    let mut i: i32 = 2;
    while i <= sqrt_n {
        if n_remaining % i == 0 {
            let mut curr_sum: i32 = 1;
            let mut curr_term: i32 = 1;
            while n_remaining % i == 0 {
                n_remaining = n_remaining.wrapping_div(i);
                curr_term = curr_term.wrapping_mul(i);
                curr_sum = curr_sum.wrapping_add(curr_term);
            }
            res = res.wrapping_mul(curr_sum);
        }
        i = i.wrapping_add(1);
    }

    if n_remaining >= 2 {
        res = res.wrapping_mul(1i32.wrapping_add(n_remaining));
    }

    res
}