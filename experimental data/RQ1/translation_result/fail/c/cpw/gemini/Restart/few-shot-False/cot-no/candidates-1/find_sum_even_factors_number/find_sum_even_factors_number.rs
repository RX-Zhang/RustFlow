
fn find_sum_even_factors_number(mut n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }
    let mut res = 1;
    for i in 2..=(n as f64).sqrt() as i32 {
        let mut count = 0;
        let mut curr_sum = 1;
        let mut curr_term = 1;
        while n % i == 0 {
            count += 1;
            n /= i;
            if i == 2 && count == 1 {
                curr_sum = 0;
            }
            curr_term *= i;
            curr_sum += curr_term;
        }
        res *= curr_sum;
    }
    if n >= 2 {
        res *= (1 + n);
    }
    res
}
