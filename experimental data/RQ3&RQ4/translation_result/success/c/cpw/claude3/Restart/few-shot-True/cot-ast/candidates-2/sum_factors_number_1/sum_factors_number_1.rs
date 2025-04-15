
fn sum_factors_number_1(mut n: i32) -> i32 {
    let mut res: i32 = 1;
    let sqrt_n = (n as f64).sqrt() as i32;
    
    for i in 2..=sqrt_n {
        let mut curr_sum: i64 = 1;
        let mut curr_term: i64 = 1;
        
        while n % i == 0 {
            n /= i;
            curr_term = curr_term.wrapping_mul(i as i64);
            curr_sum = curr_sum.wrapping_add(curr_term);
        }
        
        res = (res as i64).wrapping_mul(curr_sum) as i32;
    }
    
    if n >= 2 {
        res = (res as i64).wrapping_mul((n as i64).wrapping_add(1)) as i32;
    }
    
    res
}
