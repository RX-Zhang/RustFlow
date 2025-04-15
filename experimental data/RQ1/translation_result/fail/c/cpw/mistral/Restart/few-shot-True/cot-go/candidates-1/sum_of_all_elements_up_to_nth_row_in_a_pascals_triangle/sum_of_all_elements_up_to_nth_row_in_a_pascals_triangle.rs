
use std::i32;

fn sum_ofall_elements_up_to_nth_row_in__pascals_tri(n: i32) -> i32 {
    let mut sum: i32 = 0;
    for row in 0..n {
        sum = sum.wrapping_add((1 << row) as i32);
    }
    sum
}
