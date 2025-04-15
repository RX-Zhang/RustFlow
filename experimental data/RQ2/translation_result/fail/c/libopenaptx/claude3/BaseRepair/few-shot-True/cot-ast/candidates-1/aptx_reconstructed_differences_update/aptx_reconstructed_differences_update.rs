
use std::mem::MaybeUninit;

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

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) -> i32 {
    let (rd1, rd2) = prediction.reconstructed_differences.split_at_mut(order);
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = ((p as i32).wrapping_add(1) % order as i32).wrapping_abs();
    let new_p = prediction.pos as usize;
    rd2[new_p] = reconstructed_difference;
    rd2[new_p]
}
