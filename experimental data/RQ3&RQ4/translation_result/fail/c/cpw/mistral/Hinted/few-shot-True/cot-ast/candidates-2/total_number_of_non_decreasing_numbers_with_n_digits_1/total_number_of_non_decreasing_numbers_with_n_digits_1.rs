
use std::i32;
use std::num::Wrapping;

fn total_number_of_non_decreasing_numbers_with_n_digits_1(n: i32) -> i32 {
    let mut count = Wrapping(1);
    let mut i = 1;
    let n = n as i64;
    let N = 10;

    while i <= n {
        count *= Wrapping(N + i as i64 - 1);
        count /= Wrapping(i);
        i += 1;
    }

    count.0 as i32
}
