
fn count_factorial_numbers_in_a_given_range(low: i32, high: i32) -> i32 {
    let mut fact = 1;
    let mut x = 1;
    while fact < low {
        fact = fact.wrapping_mul(x);
        x += 1;
    }
    let mut res = 0;
    while fact <= high {
        res += 1;
        fact = fact.wrapping_mul(x);
        x += 1;
    }
    res
}
