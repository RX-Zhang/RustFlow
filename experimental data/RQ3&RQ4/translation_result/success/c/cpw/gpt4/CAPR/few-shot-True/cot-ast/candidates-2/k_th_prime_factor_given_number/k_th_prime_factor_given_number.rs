
fn k_th_prime_factor_given_number(n: i32, k: i32) -> i32 {
    let mut n = n;
    let mut k = k;

    if n == 0 {
        return 2; // Handle the special case where n is 0
    }

    while n % 2 == 0 {
        k -= 1;
        n = n.wrapping_div(2);
        if k == 0 {
            return 2;
        }
    }

    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        while n % i == 0 {
            if k == 1 {
                return i;
            }
            k -= 1;
            n = n.wrapping_div(i);
        }
    }

    if n > 2 && k == 1 {
        return n;
    }
    -1
}
