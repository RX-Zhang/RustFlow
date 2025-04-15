use std::boxed::Box;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p)) & !((2u32 << p).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1) as i32;
    let mask = (1 << (shift + 1) as i32) - 1;
    ((value + rounding) >> shift as i32) - ((value & mask) == rounding) as i32
}

fn aptx_reconstructed_differences_update(prediction: &mut Box<AptxPrediction>, reconstructed_difference: i32, order: i32) -> *mut i32 {
    let rd1 = prediction.reconstructed_differences.as_mut_ptr();
    let rd2 = unsafe { rd1.add(order as usize) };
    let p = prediction.pos;

    unsafe {
        *rd1.add(p as usize) = *rd2.add(p as usize);
    }
    prediction.pos = (p + 1) % order;
    unsafe {
        *rd2.add(prediction.pos as usize) = reconstructed_difference;
    }
    unsafe { rd2.add(prediction.pos as usize) }
}

fn aptx_prediction_filtering(prediction: &mut Box<AptxPrediction>, reconstructed_difference: i32, order: i32) {
    let reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2( (((prediction.s_weight[0] as i64) * (prediction.previous_reconstructed_sample as i64)).wrapping_add(
                                    (prediction.s_weight[1] as i64) * (reconstructed_sample as i64)) >> 22) as i32, 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences = aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = ((reconstructed_difference > 0) as i32 - (reconstructed_difference < 0) as i32) * (1 << 23);
    for i in 0..order {
        let srd = (unsafe { *reconstructed_differences.sub((i + 1) as usize) } >> 31) | 1;
        prediction.d_weight[i as usize] = prediction.d_weight[i as usize].wrapping_sub(rshift32(prediction.d_weight[i as usize].wrapping_sub(srd.wrapping_mul(srd0)), 8));
        let temp = (unsafe { *reconstructed_differences.sub(i as usize) } as i64) * (prediction.d_weight[i as usize] as i64);
        prediction.predicted_difference = prediction.predicted_difference.wrapping_add(temp as i32);
    }

    prediction.predicted_difference = clip_intp2(prediction.predicted_difference >> 22, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}


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

