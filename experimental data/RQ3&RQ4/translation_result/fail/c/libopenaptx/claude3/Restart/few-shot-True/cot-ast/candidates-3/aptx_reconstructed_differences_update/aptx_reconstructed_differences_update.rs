use std::mem;

#[derive(Debug)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

impl Default for AptxPrediction {
    fn default() -> Self {
        Self {
            prev_sign: [0; 2],
            s_weight: [0; 2],
            d_weight: [0; 24],
            pos: 0,
            reconstructed_differences: [0; 48],
            previous_reconstructed_sample: 0,
            predicted_difference: 0,
            predicted_sample: 0,
        }
    }
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) -> i32 {
    let p = prediction.pos as usize;
    let new_p = (p.wrapping_add(1)) % order;

    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[order.wrapping_add(p)];
    prediction.reconstructed_differences[order.wrapping_add(new_p)] = reconstructed_difference;
    prediction.pos = new_p as i32;

    prediction.reconstructed_differences[order.wrapping_add(new_p)]
}
