
use std::f64;

fn find_sum_even_factors_number(n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }

    let mut res: i32 = 1;
    let mut num = n;
    let mut i: i32 = 2;

    while i <= (num as f64).sqrt() as i32 {
        let mut count: i32 = 0;
        let mut curr_sum: i32 = 1;
        let mut curr_term: i32 = 1;

        while num % i == 0 {
            count = count.wrapping_add(1);
            num /= i;
            if i == 2 && count == 1 {
                curr_sum = 0;
            }
            curr_term = curr_term.wrapping_mul(i);
            curr_sum = curr_sum.wrapping_add(curr_term);
        }
        res = res.wrapping_mul(curr_sum);
        i = i.wrapping_add(1);
    }

    if num >= 2 {
        res = res.wrapping_mul(1 + num);
    }

    res
}
