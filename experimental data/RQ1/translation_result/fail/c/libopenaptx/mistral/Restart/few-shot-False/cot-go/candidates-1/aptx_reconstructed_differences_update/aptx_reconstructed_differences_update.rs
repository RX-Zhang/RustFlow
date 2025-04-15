#[allow(dead_code)]
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

#[allow(dead_code)]
fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> *mut i32 {
    let p = prediction.pos;

    prediction.reconstructed_differences[p as usize] = prediction.reconstructed_differences[(order + p) as usize];
    prediction.pos = (p.wrapping_add(1)).wrapping_rem(order);
    let p = prediction.pos;
    prediction.reconstructed_differences[order as usize + p as usize] = reconstructed_difference;

    &mut prediction.reconstructed_differences[(order + p) as usize] as *mut i32
}
