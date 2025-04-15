
fn sum_of_all_elements_up_to_nth_row_in_a_pascals_triangle(n: i32) -> i32 {
    let mut sum = 0;
    for row in 0..n {
        sum = sum + (1 << row);
    }
    sum
}
