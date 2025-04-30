use std::boxed::Box;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add((1 << p) as u32)) & !((((2 as u32) << p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = (1 << (shift - 1)) as i32;
    let mask = ((1 << (shift)) - 1) as i32;
    ((value.wrapping_add(rounding)) >> shift) - (((value & mask) == rounding) as i32)
}

fn diffsgn(x: i32, y: i32) -> i32 {
    (x > y) as i32 - (x < y) as i32
}

fn aptx_reconstructed_differences_update(prediction: &mut Box<AptxPrediction>, reconstructed_difference: i32, order: i32) -> i32 {
    let p = prediction.pos as usize;
    let order_usize = order as usize;

    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[order_usize];
    prediction.pos = (p + 1) % order_usize;
    prediction.reconstructed_differences[prediction.pos] = reconstructed_difference;
    prediction.reconstructed_differences[prediction.pos]
}

fn aptx_prediction_filtering(prediction: &mut Box<AptxPrediction>, reconstructed_difference: i32, order: i32) {
    let reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2(((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64)
        .wrapping_add(prediction.s_weight[1] as i64 * reconstructed_sample as i64) >> 22) as i32, 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_difference_updated = aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = diffsgn(reconstructed_difference, 0) * (1 << 23);

    for i in 0..order {
        let index = prediction.pos.wrapping_sub((i as usize).wrapping_add(1)) % (order as usize);
        let srd = ((prediction.reconstructed_differences[index] >> 31) | 1);
        prediction.d_weight[i as usize] = prediction.d_weight[i as usize].wrapping_sub(rshift32(prediction.d_weight[i as usize].wrapping_sub(srd * srd0), 8));

        let index2 = prediction.pos.wrapping_sub(i as usize) % (order as usize);
        prediction.predicted_difference = prediction.predicted_difference.wrapping_add(((prediction.reconstructed_differences[index2] as i64) * (prediction.d_weight[i as usize] as i64)) as i32);
    }

    prediction.predicted_difference = clip_intp2(prediction.predicted_difference >> 22, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}


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
