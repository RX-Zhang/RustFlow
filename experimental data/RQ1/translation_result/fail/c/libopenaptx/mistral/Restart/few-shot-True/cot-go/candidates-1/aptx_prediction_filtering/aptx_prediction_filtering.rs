use std::boxed::Box;

const fn diffsign(x: i32, y: i32) -> i32 {
    ((x > y) as i32) - ((x < y) as i32)
}

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

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: i32) -> &mut i32 {
    let p = prediction.pos as usize;
    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[(p + order as usize) % 48];
    prediction.pos = (p as i32 + 1) % order;
    prediction.reconstructed_differences[prediction.pos as usize] = reconstructed_difference;
    &mut prediction.reconstructed_differences[prediction.pos as usize]
}


fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: i32) {
    let reconstructed_sample: i32;
    let predictor: i32;
    let srd0: i32;
    let srd: i32;
    let _i: i32;

    reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    predictor = clip_intp2(((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64)
                            .wrapping_add(prediction.s_weight[1] as i64 * reconstructed_sample as i64) >> 22) as i32, 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);

    srd0 = diffsign(reconstructed_difference, 0) * (1 << 23);
    for i in 0..order {
        srd = ((prediction.reconstructed_differences[(prediction.pos as usize + order as usize - 1 - i as usize) % 48] >> 31) | 1);
        prediction.d_weight[i as usize] = prediction.d_weight[i as usize].wrapping_sub(rshift32(prediction.d_weight[i as usize].wrapping_sub(srd.wrapping_mul(srd0)), 8));
        prediction.predicted_difference = prediction.predicted_difference.wrapping_add(((prediction.reconstructed_differences[(prediction.pos as usize + order as usize - 1 - i as usize) % 48] as i64 * prediction.d_weight[i as usize] as i64) >> 22) as i32); //The fix is here.  We need to cast back to i32.  The original code was trying to add an i64 to an i32.
    }

    prediction.predicted_difference = clip_intp2((prediction.predicted_difference) , 23); // Removed unnecessary cast
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}
