
use std::f64;

fn find_sum_even_factors_number(mut n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }
    
    let mut res: i32 = 1;
    let sqrt_n = f64::sqrt(n as f64) as i32;
    
    for i in 2..=sqrt_n {
        let mut count = 0;
        let mut curr_sum: i32 = 1;
        let mut curr_term: i32 = 1;
        
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
