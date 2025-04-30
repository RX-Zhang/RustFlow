use std::cmp::Ordering;

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
    if ((a as u32).wrapping_add(1 << p)) & !((2u32 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask == rounding) as i32)
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let rd1 = &mut prediction.reconstructed_differences;
    let rd2 = &mut rd1[order as usize..];
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = (p as i32 + 1) % order;
    rd2[p] = reconstructed_difference;
    &mut rd2[p]
}

fn aptx_prediction_filtering(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) {
    let mut reconstructed_sample;
    let mut predictor;
    let mut srd0;
    let mut srd;
    let mut predicted_difference: i64 = 0;

    reconstructed_sample = clip_intp2(
        reconstructed_difference.wrapping_add(prediction.predicted_sample),
        23,
    );
    predictor = clip_intp2(
        (((prediction.s_weight[0] as i64) * (prediction.previous_reconstructed_sample as i64)
            + (prediction.s_weight[1] as i64) * (reconstructed_sample as i64))
            >> 22) as i32,
        23,
    );
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences =
        aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    srd0 = match reconstructed_difference.cmp(&0) {
        Ordering::Greater => 1 << 23,
        Ordering::Less => -(1 << 23),
        Ordering::Equal => 0,
    };
    for i in 0..order {
        srd = (reconstructed_differences[-i as usize - 1] >> 31) | 1;
        prediction.d_weight[i as usize] -= rshift32(
            prediction.d_weight[i as usize].wrapping_sub(srd.wrapping_mul(srd0)),
            8,
        );
        predicted_difference += (reconstructed_differences[-i as usize] as i64)
            * (prediction.d_weight[i as usize] as i64);
    }

    prediction.predicted_difference =
        clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample =
        clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}
