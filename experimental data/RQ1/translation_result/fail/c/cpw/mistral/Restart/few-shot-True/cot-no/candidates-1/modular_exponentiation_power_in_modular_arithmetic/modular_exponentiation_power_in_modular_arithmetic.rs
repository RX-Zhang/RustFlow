
use std::u32;
use std::i32;
use std::usize;
use std::ops::Shr;
use std::num::Wrapping;

fn modular_exponentiation_power_in_modular_arithmetic(x: i32, mut y: u32, p: i32) -> i32 {
    let mut res = Wrapping(1);
    let mut x_mod_p = (x % p) as u32;

    while y > 0 {
        if y & 1 == 1 {
            res = (res * Wrapping(x_mod_p as i32)) % Wrapping(p as i32);
        }
        y = y.shr(1);
        x_mod_p = (x_mod_p.wrapping_mul(x_mod_p)) % (p as u32);
    }

    res.0
}
