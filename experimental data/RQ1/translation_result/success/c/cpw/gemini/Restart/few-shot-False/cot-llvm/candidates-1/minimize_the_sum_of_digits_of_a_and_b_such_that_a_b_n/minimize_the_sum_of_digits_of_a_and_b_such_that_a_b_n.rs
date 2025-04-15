
fn minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n(n: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut n = n;
    while n > 0 {
        sum = sum.wrapping_add(n % 10);
        n = n / 10;
    }
    if sum == 1 {
        10
    } else {
        sum
    }
}
