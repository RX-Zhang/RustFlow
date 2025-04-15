
use std::num::Wrapping;

fn find_minimum_sum_factors_number(num: i32) -> i32 {
    let mut num = num;
    let mut sum = Wrapping(0);
    let mut i = 2;

    while i * i <= num {
        let mut n = num;
        while n % i == 0 {
            sum = sum + Wrapping(i);
            n /= i;
        }
        num = n;
        i += 1;
    }
    sum = sum + Wrapping(num);
    sum.0
}
