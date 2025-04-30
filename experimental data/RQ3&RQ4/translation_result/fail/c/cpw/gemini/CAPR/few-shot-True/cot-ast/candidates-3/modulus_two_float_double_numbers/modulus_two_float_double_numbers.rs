
fn modulus_two_float_double_numbers(a: f64, mut b: f64) -> f64 {
    let mut mod_val = if a < 0.0 { -a } else { a };
    if b < 0.0 {
        b = -b;
    }
    while mod_val >= b {
        mod_val -= b;
    }
    if a < 0.0 {
        -mod_val
    } else {
        mod_val
    }
}
