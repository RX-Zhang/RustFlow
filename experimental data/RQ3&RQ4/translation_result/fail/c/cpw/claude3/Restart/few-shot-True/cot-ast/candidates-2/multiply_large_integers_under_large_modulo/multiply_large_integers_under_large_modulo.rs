
use std::num::Wrapping;

fn multiply_large_integers_under_large_modulo(a: i64, b: i64, mod_val: i64) -> i64 {
    let mut res = Wrapping(0i64);
    let mut a = Wrapping(a % mod_val);
    let mut b = Wrapping(b);
    let mod_val = Wrapping(mod_val);

    while b.0 != 0 {
        if b.0 & 1 != 0 {
            res = (res + a) % mod_val;
        }
        a = (a + a) % mod_val;
        b = b >> 1;
    }

    res.0
}
