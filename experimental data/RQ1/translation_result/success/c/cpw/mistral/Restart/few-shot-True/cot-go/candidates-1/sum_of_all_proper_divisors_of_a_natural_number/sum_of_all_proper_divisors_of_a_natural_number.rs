
use std::i32;
use std::ops::Add;

fn sum_of_all_proper_divisors_of_a_natural_number(num: i32) -> i32 {
    let mut result = 0;
    let mut sqrt_num = (num as f64).sqrt() as i32;

    if sqrt_num * sqrt_num < num {
        sqrt_num += 1;
    }

    for i in 2..=sqrt_num {
        if num % i == 0 {
            let divisor_pair = (i, num / i);
            if divisor_pair.0 == divisor_pair.1 {
                result = result.add(divisor_pair.0);
            } else {
                result = result.add(divisor_pair.0).add(divisor_pair.1);
            }
        }
    }

    result.add(1)
}
