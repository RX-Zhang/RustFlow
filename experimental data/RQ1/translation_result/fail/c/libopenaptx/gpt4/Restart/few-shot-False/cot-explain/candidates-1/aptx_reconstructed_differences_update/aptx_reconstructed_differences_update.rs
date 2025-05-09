
fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let (rd1, rd2) = prediction.reconstructed_differences.split_at_mut(order as usize);
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = (p as i32 + 1).wrapping_rem(order);
    rd2[p] = reconstructed_difference;
    &mut rd2[p]
}
