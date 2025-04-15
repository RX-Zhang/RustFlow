
fn sum_of_all_elements_up_to_nth_row_in_a_pascals_triangle(n: i32) -> i32 {
    let mut sum: i64 = 0;
    for row in 0..n {
        sum = sum.wrapping_add(1i64.wrapping_shl(row as u32));
    }
    sum as i32
}
