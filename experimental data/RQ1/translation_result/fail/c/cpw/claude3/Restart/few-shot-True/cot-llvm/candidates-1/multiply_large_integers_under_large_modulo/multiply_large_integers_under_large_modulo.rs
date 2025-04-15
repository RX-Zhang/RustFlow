
fn multiply_large_integers_under_large_modulo(a: i64, b: i64, mod_val: i64) -> i64 {
    let mut res: i64 = 0;
    let mut a = a % mod_val;
    let mut b = b;

    while b != 0 {
        if b & 1 != 0 {
            res = res.wrapping_add(a).wrapping_rem(mod_val);
        }
        a = a.wrapping_mul(2).wrapping_rem(mod_val);
        b = b.wrapping_shr(1);
    }

    res
}
