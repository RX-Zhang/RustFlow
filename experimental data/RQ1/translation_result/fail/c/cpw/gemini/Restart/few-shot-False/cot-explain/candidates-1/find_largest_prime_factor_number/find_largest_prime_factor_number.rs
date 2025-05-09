
use std::f64;

fn find_largest_prime_factor_number(mut n: i64) -> i64 {
    let mut max_prime = -1;

    // Remove all 2s from n
    while n % 2 == 0 {
        max_prime = 2;
        n >>= 1;
    }

    let sqrt_n = (n as f64).sqrt() as i64;
    for i in (3..=sqrt_n).step_by(2) {
        while n % i == 0 {
            max_prime = i;
            n /= i;
        }
    }

    if n > 2 {
        max_prime = n;
    }

    max_prime
}
