
fn minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n(n: i32) -> i32 {
    let mut n = n;
    let mut sum: i32 = 0;
    while n > 0 {
        sum = sum.wrapping_add((n % 10) as i32);
        n /= 10;
    }
    if sum == 1 {
        return 10;
    }
    sum
}
