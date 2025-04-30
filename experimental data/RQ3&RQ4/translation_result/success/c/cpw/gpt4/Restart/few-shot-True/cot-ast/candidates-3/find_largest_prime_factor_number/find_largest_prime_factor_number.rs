use std::f64;

fn find_largest_prime_factor_number(n: i64) -> i64 {
    let mut max_prime: i64 = -1;
    let mut n_mut = n;

    while n_mut.wrapping_rem(2) == 0 {
        max_prime = 2;
        n_mut = n_mut.wrapping_shr(1);
    }

    let limit = (n_mut as f64).sqrt() as i64;
    let mut i: i64 = 3;
    while i <= limit {
        while n_mut.wrapping_rem(i) == 0 {
            max_prime = i;
            n_mut = n_mut.wrapping_div(i);
        }
        i = i.wrapping_add(2);
    }

    if n_mut > 2 {
        max_prime = n_mut;
    }

    max_prime
}