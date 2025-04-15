fn aptx_prediction_filtering(
    prediction: &mut AptxPrediction,
    reconstructed_difference: i32,
    order: i32,
) {
    let reconstructed_sample = clip_intp2(
        reconstructed_difference.wrapping_add(prediction.predicted_sample),
        23,
    );

    let predictor = clip_intp2(
        ((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64
            + prediction.s_weight[1] as i64 * reconstructed_sample as i64)
            >> 22) as i32,
        23,
    );

    prediction.previous_reconstructed_sample = reconstructed_sample;

    aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    let srd0 = diff_sign(reconstructed_difference, 0) * (1 << 23);

    let mut predicted_difference: i64 = 0;
    for i in 0..order {
        let srd = (prediction.reconstructed_differences[(order - i - 1) as usize] >> 31) | 1;
        prediction.d_weight[i as usize] = prediction.d_weight[i as usize]
            .wrapping_sub((srd * srd0).wrapping_shr(8));
        predicted_difference = predicted_difference.wrapping_add(
            prediction.reconstructed_differences[(order - i) as usize] as i64
                * prediction.d_weight[i as usize] as i64,
        );
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
    prediction.predicted_sample =
        clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}

fn aptx_invert_quantization(
    invert_quantize: &mut AptxInvertQuantize,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    let idx = (quantized_sample ^ ((quantized_sample < 0) as i32).wrapping_neg()) + 1;
    let mut qr = tables.quantize_intervals[idx as usize] / 2;
    if quantized_sample < 0 {
        qr = -qr;
    }

    qr = rshift64_clip24(
        ((qr as i64) << 32)
            .wrapping_add(dither as i64 * tables.invert_quantize_dither_factors[idx as usize] as i64),
        32,
    );

    invert_quantize.reconstructed_difference =
        ((invert_quantize.quantization_factor as i64 * qr as i64) >> 19) as i32;

    let mut factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(
        factor_select.wrapping_add(
            tables.quantize_factor_select_offset[idx as usize] as i32 * (1 << 15),
        ),
        15,
    );
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    let idx = (invert_quantize.factor_select & 0xFF) >> 3;
    let shift = (tables.factor_max - invert_quantize.factor_select) >> 8;
    invert_quantize.quantization_factor =
        ((QUANTIZATION_FACTORS[idx as usize] as i32) << 11) >> shift;
}
