
pub fn sum_factors_number_1(mut n: i32) -> i32 {
    let mut res: i64 = 1;

    for i in 2..=((f64::from(n).sqrt()) as i32) {
        let mut curr_sum: i64 = 1;
        let mut curr_term: i64 = 1;
        
        while n % i == 0 {
            n /= i;
            curr_term = curr_term.wrapping_mul(i as i64);
            curr_sum = curr_sum.wrapping_add(curr_term);
        }
        
        res = res.wrapping_mul(curr_sum);
    }
    
    if n >= 2 {
        res = res.wrapping_mul(1 + n as i64);
    }
    
    res as i32
}
