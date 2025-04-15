
fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i64 {
    let n = n as i64;
    let n = 10;
    let mut count: i64 = 1;

    for i in 1..=n {
        count = count.wrapping_mul(n + i - 1) / i;
    }

    count
}
