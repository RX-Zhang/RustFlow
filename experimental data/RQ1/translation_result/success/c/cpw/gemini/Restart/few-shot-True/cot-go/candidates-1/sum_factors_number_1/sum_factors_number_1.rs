
use std::num::Wrapping;

fn sum_factors_number_1(n: i32) -> i32 {
    let mut res = Wrapping(1);
    let mut n = Wrapping(n);
    let sqrt_n = (n.0 as f64).sqrt() as i32;

    for i in 2..=sqrt_n {
        let mut curr_sum = Wrapping(1);
        let mut curr_term = Wrapping(1);
        while n % Wrapping(i) == Wrapping(0) {
            n = n / Wrapping(i);
            curr_term = curr_term * Wrapping(i);
            curr_sum = curr_sum + curr_term;
        }
        res = res * curr_sum;
    }

    if n.0 >= 2 {
        res = res * (Wrapping(1) + n);
    }

    res.0
}
