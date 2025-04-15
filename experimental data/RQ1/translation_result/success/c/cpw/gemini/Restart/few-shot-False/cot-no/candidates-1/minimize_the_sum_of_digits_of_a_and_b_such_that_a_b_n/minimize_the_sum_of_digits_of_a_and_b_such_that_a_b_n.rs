
fn minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n(n: i32) -> i32 {
    let mut sum = 0;
    let mut temp = n;
    while temp > 0 {
        sum += (temp % 10) as i32;
        temp /= 10;
    }
    if sum == 1 {
        10
    } else {
        sum
    }
}
