
use std::num::Wrapping;

fn multiply_large_integers_under_large_modulo(a: i64, b: i64, mod_value: i64) -> i64 {
    let mut res = Wrapping(0);
    let mut a_mod = a % mod_value;
    let mut b_mod = b;

    while b_mod != 0 {
        if (b_mod & 1) == 1 {
            res = (res + Wrapping(a_mod)) % Wrapping(mod_value);
        }
        a_mod = (a_mod * 2) % mod_value;
        b_mod >>= 1;
    }

    res.0
}
