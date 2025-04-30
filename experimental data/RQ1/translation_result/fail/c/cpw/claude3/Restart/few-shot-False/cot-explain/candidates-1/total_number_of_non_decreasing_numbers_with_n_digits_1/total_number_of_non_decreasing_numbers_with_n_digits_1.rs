
fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i32 {
    let n = n as u32;
    let n = 10;
    let mut count: i64 = 1;
    for i in 1..=n {
        count = count.wrapping_mul((n + i - 1) as i64);
        count = count.wrapping_div(i as i64);
    }
    count as i32
}
