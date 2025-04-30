fn modulus_two_float_double_numbers(a: f64, b: f64) -> f32 {
    let mut mod_val = if a < 0.0 { -a } else { a };
    let mut b_mut = if b < 0.0 { -b } else { b };

    while mod_val >= b_mut {
        mod_val -= b_mut;
    }

    if a < 0.0 {
        return (-mod_val) as f32;
    }
    mod_val as f32
}