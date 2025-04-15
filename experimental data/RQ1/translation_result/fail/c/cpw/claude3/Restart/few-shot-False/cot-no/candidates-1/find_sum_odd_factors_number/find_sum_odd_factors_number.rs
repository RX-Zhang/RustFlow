
use std::f64;

fn find_sum_odd_factors_number(mut n: i32) -> i32 {
    let mut res: i32 = 1;
    while n % 2 == 0 {
        n = n / 2;
    }
    let sqrt_n = (n as f64).sqrt() as i32;
    for i in 3..=sqrt_n {
        let mut count: i32 = 0;
        let mut curr_sum: i32 = 1;
        let mut curr_term: i32 = 1;
        while n % i == 0 {
            count = count.wrapping_add(1);
            n = n / i;
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
