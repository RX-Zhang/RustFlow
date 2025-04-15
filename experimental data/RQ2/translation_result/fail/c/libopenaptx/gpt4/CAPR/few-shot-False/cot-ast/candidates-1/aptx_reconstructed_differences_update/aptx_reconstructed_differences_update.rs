use std::boxed::Box;

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

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) -> &mut i32 {
    let (rd1, rd2) = prediction.reconstructed_differences.split_at_mut(order);
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = ((p + 1) % order) as i32;
    rd2[p] = reconstructed_difference;
    &mut rd2[p]
}
