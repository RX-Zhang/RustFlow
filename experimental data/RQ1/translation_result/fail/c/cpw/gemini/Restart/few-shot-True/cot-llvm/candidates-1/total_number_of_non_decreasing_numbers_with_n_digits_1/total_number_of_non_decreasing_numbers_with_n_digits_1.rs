
fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i32 {
    let n = n as usize;
    let mut count = 1;
    for i in 1..=n {
        count = (count * (10 + i - 1)) / i;
    }
    count as i32
}
