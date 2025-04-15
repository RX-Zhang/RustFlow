

fn minimize_thesum_of_digits_of_a_and_b_such_that_a_b_n(n: i32) -> i32 {
    let mut sum = 0;
    let mut n = n;

    while n > 0 {
        sum += n % 10;
        n /= 10;
    }

    if sum == 1 {
        10
    } else {
        sum
    }
}
