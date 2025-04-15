
use std::i32;

fn n_th_number_whose_sum_of_digits_is_ten(n: i32) -> i32 {
    let mut count = 0;
    let mut curr = 1;
    loop {
        let mut sum = 0;
        let mut x = curr;
        while x > 0 {
            sum += x % 10;
            x /= 10;
        }
        if sum == 10 {
            count += 1;
        }
        if count == n {
            return curr;
        }
        curr += 1;
    }
}

// Note: The function will not return -1 in Rust if the loop does not terminate,
//       instead, it will loop indefinitely. In a real-world scenario, you would
//       want to add a check or a maximum limit to the loop to handle such cases.
