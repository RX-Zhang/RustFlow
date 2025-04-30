
fn find_sum_even_factors_number(n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }
    let mut res: i32 = 1;
    let mut n_copy = n;
    let sqrt_n = (f64::from(n_copy) as f64).sqrt() as i32;
    for i in 2..=sqrt_n {
        let mut count = 0;
        let mut curr_sum: i32 = 1;
        let mut curr_term: i32 = 1;
        while n_copy % i == 0 {
            count += 1;
            n_copy /= i;
            if i == 2 && count == 1 {
                curr_sum = 0;
            }
            curr_term *= i;
            curr_sum += curr_term;
        }
        res = res.wrapping_mul(curr_sum);
    }
    if n_copy >= 2 {
        res = res.wrapping_mul(1 + n_copy);
    }
    res
}
