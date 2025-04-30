
use std::i32;

fn sum_all_elements_upto_nth_rowin_a_pascals_triangle(n: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut row: i32 = 0;
    while row < n {
        sum = sum.wrapping_add((1 as i32) << row);
        row += 1;
    }
    sum
}
