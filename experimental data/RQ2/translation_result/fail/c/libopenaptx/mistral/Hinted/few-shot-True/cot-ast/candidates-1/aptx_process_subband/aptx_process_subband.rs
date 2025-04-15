use std::cmp::{max, min};

const QUANTIZATION_FACTORS: [i16; 32] = [
    2048, 2093, 2139, 2186, 2233, 2282, 2332, 2383, 2435, 2489, 2543, 2599, 2656, 2714, 2774, 2834,
    2896, 2960, 3025, 3091, 3158, 3228, 3298, 3371, 3444, 3520, 3597, 3676, 3756, 3838, 3922, 4008,
];

struct AptxTables<'a> {
    quantize_intervals: &'a [i32],
    invert_quantize_dither_factors: &'a [i32],
    quantize_dither_factors: &'a [i32],
    quantize_factor_select_offset: &'a [i16],
    tables_size: usize,
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
    if ((a as u32).wrapping_add(1 << p)) & !((2u32 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
}

fn aptx_reconstructed_differences_update(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) -> &mut i32 {
    let (rd1, rd2) = prediction.reconstructed_differences.split_at_mut(order as usize);
    let p = prediction.pos as usize;

    rd1[p] = rd2[p];
    prediction.pos = (p + 1) as i32 % order;
    rd2[p] = reconstructed_difference;
    &mut rd2[p]
}

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    min(max(a, amin), amax)
}

fn aptx_prediction_filtering(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) {
    let mut reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    let predictor = clip_intp2(
        (((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64
            + prediction.s_weight[1] as i64 * reconstructed_sample as i64) >> 22) as i32),
        23,
    );
    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences =
        aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = (reconstructed_difference.signum() * (1 << 23)) as i32;
    let mut predicted_difference = 0i64;

    for i in 0..order as usize {
        let srd = (reconstructed_differences[i] >> 31) | 1;
        prediction.d_weight[i] -= rshift32(prediction.d_weight[i] - srd * srd0, 8);
        predicted_difference += reconstructed_differences[i] as i64 * prediction.d_weight[i] as i64;
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}

fn aptx_invert_quantization(
    invert_quantize: &mut AptxInvertQuantize,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    let mut idx = (quantized_sample ^ (-(quantized_sample < 0) as i32)) + 1;
    let mut qr = tables.quantize_intervals[idx as usize] / 2;
    if quantized_sample < 0 {
        qr = -qr;
    }

    qr = rshift64_clip24(
        ((qr as i64) << 32) + (dither as i64) * (tables.invert_quantize_dither_factors[idx as usize] as i64),
        32,
    );
    invert_quantize.reconstructed_difference =
        ((invert_quantize.quantization_factor as i64 * qr as i64) >> 19) as i32;

    let mut factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(
        factor_select + (tables.quantize_factor_select_offset[idx as usize] as i32 * (1 << 15)),
        15,
    );
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    idx = ((invert_quantize.factor_select & 0xFF) >> 3) as i32;
    let shift = (tables.factor_max - invert_quantize.factor_select) >> 8;
    invert_quantize.quantization_factor =
        (QUANTIZATION_FACTORS[idx as usize] as i32) << 11 >> shift;
}

fn aptx_process_subband(
    invert_quantize: &mut AptxInvertQuantize,
    prediction: &mut AptxPrediction,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    aptx_invert_quantization(invert_quantize, quantized_sample, dither, tables);

    let sign = (invert_quantize.reconstructed_difference
        - prediction.predicted_difference)
        .signum();
    let same_sign = [
        sign * prediction.prev_sign[0],
        sign * prediction.prev_sign[1],
    ];
    prediction.prev_sign[0] = prediction.prev_sign[1];
    prediction.prev_sign[1] = sign | 1;

    let mut range = 0x100000;
    let mut sw1 = rshift32(-same_sign[1] * prediction.s_weight[1], 1);
    sw1 = (clip(sw1, -range, range) & !0xF) * 16;

    range = 0x300000;
    let weight0 = 254 * prediction.s_weight[0] + 0x800000 * same_sign[0] + sw1;
    prediction.s_weight[0] = clip(rshift32(weight0, 8), -range, range);

    range = 0x3C0000 - prediction.s_weight[0];
    let weight1 = 255 * prediction.s_weight[1] + 0xC00000 * same_sign[1];
    prediction.s_weight[1] = clip(rshift32(weight1, 8), -range, range);

    aptx_prediction_filtering(prediction, invert_quantize.reconstructed_difference, tables.prediction_order);
}
