
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

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> *mut i32 {
    let p = prediction.pos;
    let order_usize = order as usize;

    let temp_value = prediction.reconstructed_differences[p as usize];
    prediction.pos = (p.wrapping_add(1)).wrapping_rem(order);
    let new_p = prediction.pos;

    prediction.reconstructed_differences[p as usize] =
        prediction.reconstructed_differences[order_usize + p as usize];
    prediction.reconstructed_differences[order_usize + new_p as usize] =
        reconstructed_difference;

    &mut prediction.reconstructed_differences[order_usize + new_p as usize] as *mut i32
}
