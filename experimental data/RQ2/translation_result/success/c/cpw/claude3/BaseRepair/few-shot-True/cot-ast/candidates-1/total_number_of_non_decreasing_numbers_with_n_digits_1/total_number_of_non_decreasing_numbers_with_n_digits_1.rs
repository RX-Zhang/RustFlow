fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i32 {
    let N: i64 = 10;
    let mut count: i64 = 1;
    for i in 1..=n {
        count = count.wrapping_mul((N + i as i64 - 1));
        count = count.wrapping_div(i as i64);
    }
    count as i32
}