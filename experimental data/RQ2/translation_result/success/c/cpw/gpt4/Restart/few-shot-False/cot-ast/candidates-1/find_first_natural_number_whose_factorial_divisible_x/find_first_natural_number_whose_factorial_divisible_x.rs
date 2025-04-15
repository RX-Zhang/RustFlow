
fn find_first_natural_number_whose_factorial_divisible_x(x: i32) -> i32 {
    if x <= 0 {
        return 1;
    }
    let mut fact: i32 = 1;
    for j in 1..=x {
        fact = fact.wrapping_mul(j);
        if fact % x == 0 {
            return j;
        }
    }
    x
}
