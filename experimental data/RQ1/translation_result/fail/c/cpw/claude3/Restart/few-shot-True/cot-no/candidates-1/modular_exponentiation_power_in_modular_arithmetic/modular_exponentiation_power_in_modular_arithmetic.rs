
use std::num::Wrapping;

fn modular_exponentiation_power_in_modular_arithmetic(x: i32, y: u32, p: i32) -> i32 {
    let mut res = Wrapping(1);
    let mut x = Wrapping(x % p);
    let mut y = Wrapping(y);

    while y.0 > 0 {
        if y.0 & 1 != 0 {
            res = Wrapping((res.0 * x.0) % p);
        }
        y = y >> 1;
        x = Wrapping((x.0 * x.0) % p);
    }

    res.0
}
