
use std::f64;

fn find_sum_odd_factors_number(n: i32) -> i32 {
    let mut res = 1;
    let mut n = n;

    while n % 2 == 0 {
        n = n.wrapping_shr(1); // Equivalent to n / 2
    }

    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        let mut count = 0;
        let mut curr_sum = 1;
        let mut curr_term = 1;

        while n % i == 0 {
            count += 1;
            n = n / i;
            curr_term *= i;
            curr_sum += curr_term;
        }

        res *= curr_sum;
    }

    if n >= 2 {
        res *= 1 + n;
    }

    res
}
