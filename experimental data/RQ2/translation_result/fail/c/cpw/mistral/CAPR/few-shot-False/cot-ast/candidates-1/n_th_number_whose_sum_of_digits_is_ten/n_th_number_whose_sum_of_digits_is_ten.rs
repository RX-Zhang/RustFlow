
use std::usize;
use std::i32;
use std::num::Wrapping;

fn n_th_number_whose_sum_of_digits_is_ten(n: usize) -> i32 {
    let mut curr = 1;
    let mut count = 0;

    loop {
        let sum = digit_sum(curr);
        if sum == 10 {
            count += 1;
        }

        if count == n {
            return curr;
        }

        curr += 1;
    }
}

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = Wrapping(0);

    while n > 0 {
        sum += Wrapping(n % 10);
        n /= 10;
    }

    sum.0
}
