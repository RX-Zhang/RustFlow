
use std::cmp::min;
use std::i32;

fn find_sum_even_factors_number(n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }

    let mut sum: i32 = 0; // Specify the type of sum as i32
    for i in 1..=n {
        if n % i == 0 && i % 2 == 0 {
            sum = sum.wrapping_add(i);
        }
    }

    sum
}
