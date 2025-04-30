fn multiply_large_integers_under_large_modulo(a: i64, b: i64, mod_val: i64) -> i64 {
    let mut res: i64 = 0;
    let mut a_mut = a.wrapping_rem(mod_val);
    let mut b_mut = b;

    while b_mut != 0 {
        if b_mut & 1 != 0 {
            res = res.wrapping_add(a_mut).wrapping_rem(mod_val);
        }
        a_mut = a_mut.wrapping_mul(2).wrapping_rem(mod_val);
        b_mut = b_mut.wrapping_shr(1);
    }

    res
}