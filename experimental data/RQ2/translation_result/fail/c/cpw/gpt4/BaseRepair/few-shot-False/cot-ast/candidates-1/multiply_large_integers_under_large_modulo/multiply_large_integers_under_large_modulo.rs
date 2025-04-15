
fn multiply_large_integers_under_large_modulo(a: i64, b: i64, mod_: i64) -> i64 {
    let mut res: i64 = 0;
    let mut a = a % mod_;
    let mut b = b;

    while b != 0 {
        if b & 1 == 1 {
            res = res.wrapping_add(a) % mod_;
        }
        a = (2 * a).wrapping_rem(mod_);
        b = b.wrapping_shr(1);
    }
    res
}
