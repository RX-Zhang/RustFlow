use std::ops::Wrapping;

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

fn clip_intp2(a: i32, p: u32) -> i32 {
    let ua = a as u32;
    if (ua + (1 << p)) & !((2 << p) - 1) != 0 {
        ((a >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask == rounding) as i32)
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) -> &mut i32 {
    let p = prediction.pos;
    let rd1 = &mut prediction.reconstructed_differences[..order];
    let rd2 = &mut prediction.reconstructed_differences[order..];

    rd1[p] = rd2[p];
    prediction.pos = (p + 1) % order;
    rd2[prediction.pos] = reconstructed_difference;
    &mut rd2[prediction.pos]
}

fn aptx_prediction_filtering(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: usize,
) {
    let mut reconstructed_sample = clip_intp2(
        reconstructed_difference.wrapping_add(prediction.predicted_sample),
        23,
    );

    let predictor = clip_intp2(
        (((prediction.s_weight[0] as i64)
            .wrapping_mul(prediction.previous_reconstructed_sample as i64)
            .wrapping_add((prediction.s_weight[1] as i64).wrapping_mul(reconstructed_sample as i64)))
            >> 22) as i32,
        23,
    );

    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences =
        aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = DIFFSIGN(reconstructed_difference, 0) * (1 << 23);
    let mut predicted_difference: i64 = 0;

    for i in 0..order {
        let srd = (reconstructed_differences[-i - 1] >> 31) | 1;
        prediction.d_weight[i] = prediction.d_weight[i]
            - rshift32(
                prediction.d_weight[i] - srd * srd0,
                8,
            );
        predicted_difference = predicted_difference.wrapping_add(
            reconstructed_differences[-i] as i64 * prediction.d_weight[i] as i64,
        );
    }

    prediction.predicted_difference =
        clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(
        predictor.wrapping_add(prediction.predicted_difference),
        23,
    );
}

fn DIFFSIGN(x: i32, y: i32) -> i32 {
    ((x > y) as i32) - ((x < y) as i32)
}
