
#[derive(Debug)]
pub struct AptxPrediction {
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
) -> *const i32 {
    let rd1 = &mut prediction.reconstructed_differences;
    let rd2_start = rd1.as_mut_ptr();

    let rd2_ptr = unsafe { rd2_start.add(order as usize) };


    let p = prediction.pos;

    rd1[p as usize] = unsafe { *rd2_start.add(p as usize) };
    prediction.pos = (p.wrapping_add(1)).wrapping_rem(order)
    let p = prediction.pos;

    unsafe { *rd2_start.add(order as usize + p as usize) = reconstructed_difference };
    unsafe { rd2_start.add(order as usize + p as usize) }
}
