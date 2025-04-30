
fn multiply_large_integers_under_large_modulo(a: i64, b: i64, modulo: i64) -> i64 {
    let mut res: i64 = 0; // Specify the type for res as i64
    let mut a = a % modulo;
    let mut b = b;

    while b != 0 {
        if b & 1 != 0 {
            // Correct method call with specified type for res
            res = (res.wrapping_add(a)) % modulo; 
        }
        a = (2 * a) % modulo;
        b = b.wrapping_shr(1); // Use wrapping right shift
    }
    
    res
}
