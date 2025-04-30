fn aptx_reconstructed_differences_update(prediction: &mut Box<AptxPrediction>, reconstructed_difference: i32, order: i32) -> &mut i32 {
    let rd1_ptr = prediction.reconstructed_differences.as_mut_ptr();
    let rd2_ptr = unsafe { rd1_ptr.add(order as usize) };
    let p = prediction.pos;

    unsafe {
        *rd1_ptr.add(p as usize) = *rd2_ptr.add(p as usize);
        prediction.pos = (p + 1) % order
        *rd2_ptr.add(prediction.pos as usize) = reconstructed_difference;
    }
    unsafe { &mut *rd2_ptr.add(prediction.pos as usize) }
}


#[derive(Debug)]
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
