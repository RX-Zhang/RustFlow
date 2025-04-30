fn minimize_the_sum_of_digits_of_a_and_b_such_that_a_b_n(n: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut n_mut = n;

    while n_mut > 0 {
        sum = sum.wrapping_add(n_mut.wrapping_rem(10));
        n_mut = n_mut.wrapping_div(10);
    }

    if sum == 1 {
        return 10;
    }

    sum
}