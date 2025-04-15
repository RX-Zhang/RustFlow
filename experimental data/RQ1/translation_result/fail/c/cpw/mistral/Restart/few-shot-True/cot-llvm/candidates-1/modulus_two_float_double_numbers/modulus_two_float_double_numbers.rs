
use std::u32;
use std::i32;

fn modulus_two_float_double_numbers(a: f64, b: f64) -> f64 {
    let modulus = if a < 0.0 { -a } else { a };
    let b = if b < 0.0 { -b } else { b };

    let (mut a_bits, mut b_bits) = (a.to_bits(), b.to_bits());

    // Set the sign bit to 0
    a_bits &= !(1 << 63);
    b_bits &= !(1 << 63);

    let mut modulo = a_bits as u64;

    while modulo >= (b_bits as u64) {
        modulo = modulo.wrapping_sub(b_bits as u64);
    }

    let mut result_bits = (modulo as u64) as i64;

    // Set the sign bit based on the input sign
    if a < 0.0 {
        result_bits |= (1 << 63);
    }

    // Scaling the result back to the float64 range
    let scale_factor = (1 as f64) * (2 as f64).powi(52);
    let scale_factor_inv = 1.0 / scale_factor;

    result_bits as f64 * scale_factor_inv * (b.is_sign_positive() as i32) as f64
}
