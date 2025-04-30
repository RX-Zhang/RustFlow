struct AptxPrediction {
    prev_sign: [i32 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn aptx_reconstructed_differences_update(
    prediction: &mut Box<AptxPrediction>,
    reconstructed_difference: i32,
    order: i32,
) -> i32 {
    let rd1 = &mut prediction.reconstructed_differences as *mut [i32; 48] as *mut i32;
    let rd2 = unsafe { rd1.offset(order as isize) };
    let p = prediction.pos as usize;
    
    unsafe {
        *rd1.offset(p as isize) = *rd2.offset(p as isize);
    }
    
    prediction.pos = (p as i32).wrapping_add(1) % order;
    let new_pos = prediction.pos as usize;
    
    unsafe {
        *rd2.offset(new_pos as isize) = reconstructed_difference;
        *rd2.offset(new_pos as isize)
    }
}
