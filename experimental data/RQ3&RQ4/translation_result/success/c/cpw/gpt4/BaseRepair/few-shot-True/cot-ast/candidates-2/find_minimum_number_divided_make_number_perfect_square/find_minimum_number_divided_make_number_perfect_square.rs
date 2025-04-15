
use std::f64;

fn find_minimum_number_divided_make_number_perfect_square(n: i32) -> i32 {
    let mut count = 0;
    let mut ans = 1;
    let mut num = n;

    while num % 2 == 0 {
        count += 1;
        num /= 2;
    }
    if count % 2 != 0 {
        ans *= 2;
    }

    let limit = (num as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        count = 0;
        while num % i == 0 {
            count += 1;
            num /= i;
        }
        if count % 2 != 0 {
            ans *= i;
        }
    }

    if num > 2 {
        ans *= num;
    }

    ans
}
