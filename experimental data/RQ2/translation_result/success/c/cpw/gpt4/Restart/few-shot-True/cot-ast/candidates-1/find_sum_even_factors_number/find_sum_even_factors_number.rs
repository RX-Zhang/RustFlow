
fn find_sum_even_factors_number(mut n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }
    let mut res = 1i32;
    let limit = (n as f64).sqrt() as i32;

    for i in 2..=limit {
        let mut count = 0i32;
        let mut curr_sum = 1i32;
        let mut curr_term = 1i32;

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
    }
    if n >= 2 {
        res = res.wrapping_mul(1 + n);
    }
    res
}
