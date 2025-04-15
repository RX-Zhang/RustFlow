use std::mem::transmute;

const QUANTIZATION_FACTORS: [i16; 32] = [
    2048, 2093, 2139, 2186, 2233, 2282, 2332, 2383,
    2435, 2489, 2543, 2599, 2656, 2714, 2774, 2834,
    2896, 2960, 3025, 3091, 3158, 3228, 3298, 3371,
    3444, 3520, 3597, 3676, 3756, 3838, 3922, 4008,
];

struct AptxTables {
    quantize_intervals: *const i32,
    invert_quantize_dither_factors: *const i32,
    quantize_dither_factors: *const i32,
    quantize_factor_select_offset: *const i16,
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

fn diffsgn(x: i32, y: i32) -> i32 {
    (x > y) as i32 - (x < y) as i32
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32: u32 = unsafe { transmute(a) };
    if (a_u32 + (1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1 << (shift - 1);
    let mask: i64 = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding: i32 = 1 << (shift - 1);
    let mask: i32 = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
}

fn aptx_reconstructed_differences_update(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: i32) -> *mut i32 {
    let rd1 = &mut prediction.reconstructed_differences[0] as *mut i32;
    let rd2 = unsafe { rd1.add(order as usize) };
    let p = prediction.pos as usize;

    unsafe { *rd1.add(p) = *rd2.add(p); }
    prediction.pos = (p as i32 + 1) % order;
    let ptr = unsafe { rd2.add(prediction.pos as usize) };
    unsafe { *ptr = reconstructed_difference; }
    ptr
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


fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: i32) {
    let mut reconstructed_sample: i32;
    let mut predictor: i32;
    let mut srd0: i32;
    let mut srd: i32;
    let reconstructed_differences: *mut i32;
    let mut predicted_difference: i64 = 0;
    let mut i: i32;

    reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
    predictor = clip_intp2( ((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64)
                            .wrapping_add(prediction.s_weight[1] as i64 * reconstructed_sample as i64))
                            .wrapping_shr(22) as i32, 23);
    prediction.previous_reconstructed_sample = reconstructed_sample;

    reconstructed_differences = aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    srd0 = diffsgn(reconstructed_difference, 0) * (1 << 23);
    for i in 0..order {
        srd = (unsafe { *reconstructed_differences.offset(-(i+1) as isize) } >> 31) | 1;
        prediction.d_weight[i as usize] = prediction.d_weight[i as usize].wrapping_sub(rshift32(prediction.d_weight[i as usize].wrapping_sub(srd.wrapping_mul(srd0)), 8));
        predicted_difference = predicted_difference.wrapping_add(unsafe { *reconstructed_differences.offset(-i as isize) } as i64 * prediction.d_weight[i as usize] as i64);
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}

fn aptx_invert_quantization(invert_quantize: &mut AptxInvertQuantize, quantized_sample: i32, dither: i32, tables: &AptxTables) {
    let mut qr: i32;
    let idx: i32;
    let mut factor_select: i32;

    idx = (quantized_sample ^ -(quantized_sample < 0)) + 1;
    qr = unsafe { *tables.quantize_intervals.add(idx as usize) } / 2;
    if quantized_sample < 0 {
        qr = -qr;
    }

    qr = rshift64_clip24(((qr as i64) * (1 << 32) as i64).wrapping_add((dither as i64) * unsafe { *tables.invert_quantize_dither_factors.add(idx as usize) } as i64), 32);
    invert_quantize.reconstructed_difference = ((invert_quantize.quantization_factor as i64) * (qr as i64)) >> 19;

    factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(factor_select.wrapping_add((unsafe { *tables.quantize_factor_select_offset.add(idx as usize) } * (1 << 15)) ), 15);
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    idx = (invert_quantize.factor_select & 0xFF) >> 3;
    let shift = (tables.factor_max - invert_quantize.factor_select) >> 8;
    invert_quantize.quantization_factor = (QUANTIZATION_FACTORS[idx as usize] as i32 << 11) >> shift;
}

fn aptx_process_subband(invert_quantize: &mut AptxInvertQuantize, prediction: &mut AptxPrediction, quantized_sample: i32, dither: i32, tables: &AptxTables) {
    let mut sign: i32;
    let mut same_sign: [i32; 2];
    let mut weight: [i32; 2];
    let mut sw1: i32;
    let mut range: i32;

    aptx_invert_quantization(invert_quantize, quantized_sample, dither, tables);

    sign = diffsgn(invert_quantize.reconstructed_difference, -prediction.predicted_difference);
    same_sign[0] = sign.wrapping_mul(prediction.prev_sign[0]);
    same_sign[1] = sign.wrapping_mul(prediction.prev_sign[1]);
    prediction.prev_sign[0] = prediction.prev_sign[1];
    prediction.prev_sign[1] = sign | 1;

    range = 0x100000;
    sw1 = rshift32(-same_sign[1].wrapping_mul(prediction.s_weight[1]), 1);
    sw1 = (clip(sw1, -range, range) & !0xF) * 16;

    range = 0x300000;
    weight[0] = 254 * prediction.s_weight[0] + 0x800000 * same_sign[0] + sw1;
    prediction.s_weight[0] = clip(rshift32(weight[0], 8), -range, range);

    range = 0x3C0000 - prediction.s_weight[0];
    weight[1] = 255 * prediction.s_weight[1] + 0xC00000 * same_sign[1];
    prediction.s_weight[1] = clip(rshift32(weight[1], 8), -range, range);

    aptx_prediction_filtering(prediction, invert_quantize.reconstructed_difference, tables.prediction_order);
}
