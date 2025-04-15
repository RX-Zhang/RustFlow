
fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i64 {
    let N = 10;
    let mut count: i64 = 1;
    for i in 1..=n {
        count = count * (N + i as i64 - 1) / i as i64;
    }
    count
}
