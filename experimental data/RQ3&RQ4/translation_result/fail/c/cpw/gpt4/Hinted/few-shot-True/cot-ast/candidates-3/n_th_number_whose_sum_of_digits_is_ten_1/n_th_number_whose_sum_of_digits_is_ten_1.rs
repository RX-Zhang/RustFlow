
use std::num::Wrapping;

fn n_th_number_whose_sum_of_digits_is_ten_1(n: i32) -> i32 {
    let mut count = Wrapping(0);
    let mut curr = Wrapping(19);

    loop {
        let mut sum = Wrapping(0);
        let mut x = curr;

        while x > Wrapping(0) {
            sum += x % Wrapping(10);
            x = x / Wrapping(10);
        }

        if sum == Wrapping(10) {
            count += Wrapping(1);
        }

        if count == Wrapping(n) {
            return curr.0;
        }

        curr += Wrapping(9);
    }

    -1 // Should never reach here due to the loop
}
