
fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i32 {
    let n: i64 = n.into();
    let mut count: i64 = 1;
    let N: i64 = 10;
    for i in 1..=n {
        count = count.wrapping_mul(N + i - 1);
        count = count.wrapping_div(i);
    }
    count as i32
}
