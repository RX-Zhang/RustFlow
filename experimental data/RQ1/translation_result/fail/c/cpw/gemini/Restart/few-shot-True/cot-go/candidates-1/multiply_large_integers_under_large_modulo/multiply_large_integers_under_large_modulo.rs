
fn multiply_large_integers_under_large_modulo(a: i64, b: i64, mod_: i64) -> i64 {
    let mut res = 0;
    let mut a = a % mod_;
    let mut b = b;
    while b != 0 {
        if b & 1 != 0 {
            res = (res + a) % mod_;
        }
        a = (2 * a) % mod_;
        b >>= 1;
    }
    res
}
