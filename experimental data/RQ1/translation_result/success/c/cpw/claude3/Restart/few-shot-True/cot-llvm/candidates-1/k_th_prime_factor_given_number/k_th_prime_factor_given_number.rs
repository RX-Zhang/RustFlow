
fn k_th_prime_factor_given_number(mut n: i32, mut k: i32) -> i32 {
    while n % 2 == 0 {
        k = k.wrapping_sub(1);
        n = n.wrapping_div(2);
        if k == 0 {
            return 2;
        }
    }
    
    let mut i = 3;
    while i <= (n as f64).sqrt() as i32 {
        while n % i == 0 {
            if k == 1 {
                return i;
            }
            k = k.wrapping_sub(1);
            n = n.wrapping_div(i);
        }
        i = i.wrapping_add(2);
    }
    
    if n > 2 && k == 1 {
        return n;
    }
    
    -1
}
