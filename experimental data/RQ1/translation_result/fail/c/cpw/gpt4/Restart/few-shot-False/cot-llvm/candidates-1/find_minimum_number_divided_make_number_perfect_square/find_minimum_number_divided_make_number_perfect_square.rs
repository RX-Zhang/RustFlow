
use std::f64;

fn find_minimum_number_divided_make_number_perfect_square(n: i32) -> i32 {
    let mut n = n;
    let mut count = 0;
    let mut ans = 1;

    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }
    if count % 2 != 0 {
        ans *= 2;
    }
    
    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        if count % 2 != 0 {
            ans *= i;
        }
    }
    if n > 2 {
        ans *= n;
    }
    ans
}
