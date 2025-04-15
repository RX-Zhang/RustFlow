
fn minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10) as i32;
        n /= 10;
    }
    if sum == 1 {
        return 10;
    }
    sum
}
