
use std::f64;

fn k_th_prime_factor_given_number(n: i32, k: i32) -> i32 {
    let mut n = n;
    let mut k = k;

    // Handle the case where n is divisible by 2
    while n % 2 == 0 {
        k -= 1;
        n /= 2;
        if k == 0 {
            return 2;
        }
    }

    // Check for other prime factors
    for i in (3..=(n as f64).sqrt() as i32).step_by(2) {
        while n % i == 0 {
            k -= 1;
            n /= i;
            if k == 1 {
                return i;
            }
        }
    }

    // If n itself is a prime and k is 1
    if n > 2 && k == 1 {
        return n;
    }

    // If no k-th prime factor is found
    -1
}
