
use std::usize;
use std::i32;
use std::iter;

fn n_th_number_whose_sum_of_digits_is_ten_1(n: usize) -> i32 {
    let mut curr = 19;

    iter::from_fn(move || {
        if (curr as f32 / 10.0).fract() > 0.1 {
            curr += 9;
            return Some(curr);
        }

        let mut sum = 0;
        let mut x = curr;

        while x > 0 {
            sum += x % 10;
            x /= 10;
        }

        if sum == 10 {
            return Some(curr);
        }

        None
    })
    .filter(|_| true)
    .nth(n - 1)
    .unwrap_or(-1)
}
