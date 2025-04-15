
fn k_th_prime_factor_given_number(mut n: i32, mut k: i32) -> i32 {
    // Handle edge case of n=0
    if n == 0 {
        return 2;
    }

    // Take absolute value of n and k
    n = n.abs();
    k = k.abs();

    while n % 2 == 0 {
        k -= 1;
        n /= 2;
        if k == 0 {
            return 2;
        }
    }

    let sqrt_n = (n as f64).sqrt() as i32;
    let mut i = 3;
    while i <= sqrt_n {
        while n % i == 0 {
            if k == 1 {
                return i;
            }
            k -= 1; 
            n /= i;
        }
        i += 2;
    }

    if n > 2 && k == 1 {
        return n;
    }
    -1
}
