
use std::usize;
use std::ops::Shr;

fn n_th_number_whose_sum_of_digits_is_ten(n: usize) -> i32 {
    let mut curr = 1;
    let mut count = 0;

    loop {
        let sum = digit_sum(&curr);
        if sum == 10 {
            count += 1;
        }
        if count == n {
            return curr;
        }
        curr += 1;
    }
}

fn digit_sum(num: &i32) -> i32 {
    let mut sum = 0;
    let mut num_copy = *num;

    while num_copy != 0 {
        sum += num_copy % 10;
        num_copy = num_copy.shr(1);
    }

    sum
}
