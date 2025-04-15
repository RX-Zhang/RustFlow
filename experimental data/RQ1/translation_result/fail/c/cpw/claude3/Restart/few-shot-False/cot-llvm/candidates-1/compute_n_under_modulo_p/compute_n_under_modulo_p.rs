
fn compute_n_under_modulo_p(n: i32, p: i32) -> i32 {
    if n >= p {
        return 0;
    }
    
    let mut result: i32 = 1;
    for i in 1..=n {
        // Use wrapping_mul for C-like overflow behavior
        result = (result.wrapping_mul(i)) % p;
    }
    result
}
