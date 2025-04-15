
use std::i32;
use std::i64;

fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i64 {
    let mut count = 1 as i64;
    let n = n as i32;
    let n9 = 9 * n as i64;
    for i in 1..=n {
        count = (count * (n9 + i as i64)) / (i as i64);
    }
    count
}
