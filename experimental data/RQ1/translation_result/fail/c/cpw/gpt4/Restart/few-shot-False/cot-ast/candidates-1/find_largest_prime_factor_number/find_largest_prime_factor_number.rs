
use std::f64;

fn find_largest_prime_factor_number(mut n: i64) -> i64 {
    let mut max_prime: i64 = -1;
    
    while n % 2 == 0 {
        max_prime = 2;
        n = n.wrapping_shr(1);
    }
    
    let mut i = 3;
    while (i as f64) <= (n as f64).sqrt() {
        while n % i == 0 {
            max_prime = i;
            n /= i;
        }
        i += 2;
    }
    
    if n > 2 {
        max_prime = n;
    }
    
    max_prime
}
