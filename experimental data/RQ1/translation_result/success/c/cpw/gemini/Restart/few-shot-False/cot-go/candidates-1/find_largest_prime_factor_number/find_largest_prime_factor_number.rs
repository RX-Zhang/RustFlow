
use std::f64;

fn find_largest_prime_factor_number(mut n: i64) -> i64 {
    let mut max_prime = -1;
    while n % 2 == 0 {
        max_prime = 2;
        n = n.wrapping_shr(1);
    }
    for i in (3..=((n as f64).sqrt() as i64)).step_by(2) {
        while n % i == 0 {
            max_prime = i;
            n = n / i;
        }
    }
    if n > 2 {
        max_prime = n;
    }
    max_prime
}
