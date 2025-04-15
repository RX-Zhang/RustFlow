
use std::num::Wrapping;

pub fn find_largest_prime_factor_number(n: i64) -> i64 {
    let mut n = Wrapping(n);
    let mut max_prime = -1;

    while n.0 % 2 == 0 {
        max_prime = 2;
        n = Wrapping(n.0 >> 1);
    }

    let mut i = Wrapping(3);
    while i.0 * i.0 <= n.0 {
        while n.0 % i.0 == 0 {
            max_prime = i.0 as i64;
            n = Wrapping(n.0 / i.0);
        }
        i = Wrapping(i.0 + 2);
    }

    if n.0 > 2 {
        max_prime = n.0 as i64;
    }

    max_prime
}
