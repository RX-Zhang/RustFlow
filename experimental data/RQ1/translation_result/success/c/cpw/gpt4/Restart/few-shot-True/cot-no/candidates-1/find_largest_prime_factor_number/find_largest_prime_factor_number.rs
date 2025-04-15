
use std::f64;

fn find_largest_prime_factor_number(n: i64) -> i64 {
    let mut max_prime = -1;
    let mut num = n;

    while num % 2 == 0 {
        max_prime = 2;
        num = num.wrapping_shr(1);
    }

    let limit = (num as f64).sqrt() as i64;
    for i in (3..=limit).step_by(2) {
        while num % i == 0 {
            max_prime = i;
            num /= i;
        }
    }

    if num > 2 {
        max_prime = num;
    }

    max_prime
}
