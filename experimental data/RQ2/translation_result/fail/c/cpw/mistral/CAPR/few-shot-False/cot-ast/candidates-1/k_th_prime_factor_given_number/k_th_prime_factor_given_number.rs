
use std::num::Wrapping;

fn k_th_prime_factor_given_number(n: i32, k: i32) -> i32 {
    if n <= 1 || k <= 0 {
        return -1;
    }
    let mut n = Wrapping(n);
    let mut k = Wrapping(k);

    while n.0 % 2 == 0 {
        k.0 -= 1;
        n.0 = n.0.wrapping_div(2);

        if k.0 == 0 {
            return 2;
        }
    }

    let mut i = Wrapping(3);
    while i.0 * i.0 <= n.0 {
        while n.0 % i.0 == 0 {
            if k.0 == 1 {
                return i.0 as i32;
            }
            k.0 -= 1;
        }
        i.0 += 2;
    }

    -1
}
