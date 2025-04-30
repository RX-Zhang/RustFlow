
fn find_first_natural_number_whose_factorial_divisible_x(x: i32) -> i32 {
    let mut i: i32 = 1;
    let mut fact: i32 = 1;
    while i < x {
        fact = fact.wrapping_mul(i);
        if fact % x == 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    i
}
