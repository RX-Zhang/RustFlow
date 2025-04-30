use std::f64;

fn k_th_prime_factor_given_number(n: i32, k: i32) -> i32 {
    let mut k_mut = k;
    let mut n_mut = n;

    while n_mut.wrapping_rem(2) == 0 {
        k_mut = k_mut.wrapping_sub(1);
        n_mut = n_mut.wrapping_div(2);
        if k_mut == 0 {
            return 2;
        }
    }

    let limit = (n_mut as f64).sqrt() as i32;
    let mut i: i32 = 3;
    while i <= limit {
        while n_mut.wrapping_rem(i) == 0 {
            if k_mut == 1 {
                return i;
            }
            k_mut = k_mut.wrapping_sub(1);
            n_mut = n_mut.wrapping_div(i);
        }
        i = i.wrapping_add(2);
    }

    if n_mut > 2 && k_mut == 1 {
        return n_mut;
    }

    -1
}