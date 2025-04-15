
fn count_factorial_numbers_in_a_given_range(low: i32, high: i32) -> i32 {
    let mut fact = 1i32;
    let mut x = 1i32;
    
    while fact < low {
        fact = fact.wrapping_mul(x);
        x = x.wrapping_add(1);
    }
    
    let mut res = 0i32;
    while fact <= high {
        res = res.wrapping_add(1);
        fact = fact.wrapping_mul(x);
        x = x.wrapping_add(1);
    }
    
    res
}
