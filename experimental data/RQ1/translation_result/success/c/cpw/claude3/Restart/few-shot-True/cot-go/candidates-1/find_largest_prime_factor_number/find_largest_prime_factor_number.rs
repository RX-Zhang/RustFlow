

fn find_largest_prime_factor_number(mut n: i64) -> i64 {
    let mut max_prime = -1;

    while n.wrapping_rem(2) == 0 {
        max_prime = 2;
        n = n.wrapping_shr(1);
    }

    let sqrt_n = (n as f64).sqrt() as i64;
    for i in (3..=sqrt_n).step_by(2) {
        while n.wrapping_rem(i) == 0 {
            max_prime = i;
            n = n.wrapping_div(i);
        }
    }

    if n > 2 {
        max_prime = n;
    }

    max_prime
}
