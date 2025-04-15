
fn modular_exponentiation_power_in_modular_arithmetic(x: i32, mut y: u32, p: i32) -> i32 {
    let mut res = 1i32;
    let mut x = x % p;

    while y > 0 {
        if y & 1 > 0 {
            res = (res.wrapping_mul(x)).wrapping_rem(p);
        }
        y = y.wrapping_shr(1);
        x = (x.wrapping_mul(x)).wrapping_rem(p);
    }

    res
}
