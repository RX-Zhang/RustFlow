use std::cmp::{max, min};

const QUANTIZATION_FACTORS: [i16; 32] = [
    2048, 2093, 2139, 2186, 2233, 2282, 2332, 2383,
    2435, 2489, 2543, 2599, 2656, 2714, 2774, 2834,
    2896, 2960, 3025, 3091, 3158, 3228, 3298, 3371,
    3444, 3520, 3597, 3676, 3756, 3838, 3922, 4008,
];

struct AptxTables<'a> {
    quantize_intervals: &'a [i32],
    invert_quantize_dither_factors: &'a [i32],
    quantize_dither_factors: &'a [i32],
    quantize_factor_select_offset: &'a [i16],
    tables_size: i32,
    factor_max: i32,
    prediction_order: i32,
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

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p) & !((2 << p) - 1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding) >> shift) - ((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding) >> shift) - ((value & mask) == rounding) as i32)
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

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    if a < amin {
        amin
    } else if a > amax {
        amax
    } else {
        a
    }
}

fn aptx_prediction_filtering(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) {
    let mut reconstructed_sample = clip_intp2(
        reconstructed_difference.wrapping_add(prediction.predicted_sample),
        23,
    );
    let predictor = clip_intp2(
        (((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64)
            .wrapping_add(prediction.s_weight[1] as i64 * reconstructed_sample as i64))
            >> 22) as i32,
        23,
    );
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences =
        aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = (reconstructed_difference.signum() * (1 << 23)) as i32;
    let mut predicted_difference = 0i64;

    for i in 0..order {
        let srd = (reconstructed_differences[i as usize] * (1 << 23)) as i32;
        predicted_difference = predicted_difference.wrapping_add(
            (prediction.d_weight[i as usize] as i64 * srd as i64) >> 23,
        );
    }

    prediction.predicted_difference = clip_intp2(predicted_difference as i32, 23);
}
