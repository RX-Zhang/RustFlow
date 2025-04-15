
fn minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n(n: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut num = n;

    while num > 0 {
        sum = sum.wrapping_add(num % 10);
        num /= 10;
    }

    if sum == 1 {
        return 10;
    }
    
    sum
}
