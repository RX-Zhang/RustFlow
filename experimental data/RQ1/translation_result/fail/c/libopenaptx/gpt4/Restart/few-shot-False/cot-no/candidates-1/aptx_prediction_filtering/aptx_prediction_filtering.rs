
use std::num::Wrapping;

#[derive(Default)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 24], // 改为24以满足Default trait
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn diff_sign(x: i32, y: i32) -> i32 {
    (x > y) as i32 - (x < y) as i32
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a.wrapping_add(1 << p) as u32) & !((2u32 << p).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift).wrapping_sub((value & mask == rounding) as i32)
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) -> i32 {
    let p = prediction.pos as usize;
    prediction.reconstructed_differences[p] = prediction.reconstructed_differences[(p + order) % 24];
    prediction.pos = ((p + 1) % order) as i32;
    prediction.reconstructed_differences[(p + order) % 24] = reconstructed_difference;
    reconstructed_difference
}

fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: usize) {
    let reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2(
        ((Wrapping(prediction.s_weight[0] as i64) * Wrapping(prediction.previous_reconstructed_sample as i64)
            + Wrapping(prediction.s_weight[1] as i64) * Wrapping(reconstructed_sample as i64))
            .0 >> 22) as i32,
        23,
    );
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_difference = aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = diff_sign(reconstructed_difference, 0) * (1 << 23);
    
    let mut predicted_difference = 0i64;
    for i in 0..order {
        let srd = (prediction.reconstructed_differences[order - i - 1] >> 31) | 1;
        prediction.d_weight[i] = prediction.d_weight[i].wrapping_sub(rshift32(
            prediction.d_weight[i].wrapping_sub(srd.wrapping_mul(srd0)),
            8,
        ));
        predicted_difference = predicted_difference.wrapping_add(
            (prediction.reconstructed_differences[order - i - 1] as i64).wrapping_mul(prediction.d_weight[i] as i64)
        );
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}
