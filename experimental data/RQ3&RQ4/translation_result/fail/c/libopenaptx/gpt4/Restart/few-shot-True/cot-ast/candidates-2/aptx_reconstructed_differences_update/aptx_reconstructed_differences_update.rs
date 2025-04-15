struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: usize,
    reconstructed_differences: [i32; 48],
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
    let p = prediction.pos;

    rd1[p] = rd2[p];
    let p_new = (p.wrapping_add(1)) % order;
    prediction.pos = p_new;
    rd2[p_new] = reconstructed_difference;
    &mut rd2[p_new]
}
