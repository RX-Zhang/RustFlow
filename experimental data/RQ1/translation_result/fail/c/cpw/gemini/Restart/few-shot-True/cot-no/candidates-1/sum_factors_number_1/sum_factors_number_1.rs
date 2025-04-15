
use std::f64;

fn sum_factors_number_1(n: i32) -> i32 {
    let mut res = 1;
    let mut n_mut = n;
    for i in 2..=(n as f64).sqrt() as i32 {
        let mut curr_sum = 1;
        let mut curr_term = 1;
        while n_mut % i == 0 {
            n_mut /= i;
            curr_term *= i;
            curr_sum += curr_term;
        }
        res *= curr_sum;
    }
    if n_mut >= 2 {
        res *= 1 + n_mut;
    }
    res
}
