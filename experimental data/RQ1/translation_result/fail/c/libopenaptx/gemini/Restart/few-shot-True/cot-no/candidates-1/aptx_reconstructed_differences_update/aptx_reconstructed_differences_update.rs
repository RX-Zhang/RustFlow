
use std::num::Wrapping;

#[derive(Default)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: Box<[i32; 48]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: i32) -> i32 {
    let rd1 = &mut prediction.reconstructed_differences[..order as usize];
    let rd2 = &mut prediction.reconstructed_differences[order as usize..];
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = (Wrapping(prediction.pos) + Wrapping(1)).0 % order;
    let new_p = prediction.pos as usize;
    rd2[new_p] = reconstructed_difference;
    rd2[new_p]
}
