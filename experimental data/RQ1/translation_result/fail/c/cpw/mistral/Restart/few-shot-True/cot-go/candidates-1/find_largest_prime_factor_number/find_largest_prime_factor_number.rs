
use std::i64;
use std::ops::Shr;

fn find_largest_factor_number(n: i64) -> i64 {
    let mut max_prime = -1;
    let mut nn = n;
    while nn % 2 == 0 {
        max_prime = 2;
        nn = nn.shr(1);
    }
    let mut i = 3;
    while i <= (nn as f64).sqrt() as i64 {
        while nn % i == 0 {
            max_prime = i;
            nn = nn / i;
        }
        i += 2;
    }
    if nn > 2 {
        max_prime = nn;
    }
    max_prime
}
