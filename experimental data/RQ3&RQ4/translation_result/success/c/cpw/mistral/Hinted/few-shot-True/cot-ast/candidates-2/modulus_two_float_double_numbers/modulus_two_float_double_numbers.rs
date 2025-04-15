
use std::u32;
use std::ops::*;

fn modulus_two_float_doublenumbers(a: f64, b: f64) -> f64 {
    let modulus = if a < 0.0 {
        -a
    } else {
        a
    };

    let mut result = modulus as u32;
    result = result.wrapping_sub(b as u32);
    result = result.wrapping_add(b as u32).saturating_sub(b as u32);

    if a < 0.0 {
        -(result as f64)
    } else {
        result as f64
    }
}
