
fn k_th_prime_factor_given_number(n: i32, k: i32) -> i32 {
    let mut n = n;
    let mut k = k;
    while n % 2 == 0 {
        k -= 1;
        n /= 2;
        if k == 0 {
            return 2;
        }
    }
    let mut i = 3;
    while i * i <= n {
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
