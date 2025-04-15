
fn find_first_natural_number_whose_factorial_divisible_x(x: i32) -> i32 {
    let mut i = 1;
    let mut fact: i64 = 1;
    
    while i < x {
        fact = fact.wrapping_mul(i as i64);
        if (fact % x as i64) == 0 {
            return i;
        }
        i += 1;
    }
    
    i
}
