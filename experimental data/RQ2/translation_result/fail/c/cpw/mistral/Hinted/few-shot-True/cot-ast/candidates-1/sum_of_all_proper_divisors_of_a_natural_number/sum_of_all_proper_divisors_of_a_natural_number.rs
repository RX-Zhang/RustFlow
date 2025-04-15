
use std::ops::Add;

fn sum_of_all_proper_divisors_of_a_natural_number(num: i32) -> i32 {
    let mut result = 1;
    let sqrt_num = (num as f64).sqrt() as i32;

    for i in 2..=sqrt_num {
        if num % i != 0 {
            continue;
        }

        let divisor_sum = i.add(num / i);
        result = result.add(if i == num / i { i } else { divisor_sum });
    }

    result
}
