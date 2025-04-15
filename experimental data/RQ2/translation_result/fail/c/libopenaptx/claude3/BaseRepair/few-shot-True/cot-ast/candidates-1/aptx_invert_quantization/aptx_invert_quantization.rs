use std::num::Wrapping;

fn invert_quantize(quantization_factor: &mut i32, idx: usize, shift: u32) {
    *quantization_factor = ((Wrapping(QUANTIZATION_FACTORS[idx] as i32) << 11) >> shift).0;
}

const QUANTIZATION_FACTORS: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7]; // Exemple de valeurs
