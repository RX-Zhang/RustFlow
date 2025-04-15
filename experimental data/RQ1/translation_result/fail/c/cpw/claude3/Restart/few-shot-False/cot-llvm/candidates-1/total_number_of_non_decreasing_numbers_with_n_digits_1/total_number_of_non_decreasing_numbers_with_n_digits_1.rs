
fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i32 {
    let n = n as u32;
    let mut count: i64 = 1;
    let n_big = 10_i64;

    for i in 1..=n {
        count = count.wrapping_mul(n_big + i as i64 - 1).wrapping_div(i as i64);
    }

    count as i32
}
