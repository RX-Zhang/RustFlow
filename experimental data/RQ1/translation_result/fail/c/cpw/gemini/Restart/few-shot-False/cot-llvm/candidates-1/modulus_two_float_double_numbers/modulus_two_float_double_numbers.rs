
fn modulus_two_float_double_numbers(a: f64, b: f64) -> f32 {
    let mut mod_value = if a < 0.0 { -a } else { a };
    let b_abs = b.abs();

    while mod_value >= b_abs {
        mod_value -= b_abs;
    }

    if a < 0.0 {
        -mod_value as f32
    } else {
        mod_value as f32
    }
}
