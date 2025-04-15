fn aptx_process_subband(
    invert_quantize: &mut AptxInvertQuantize,
    prediction: &mut AptxPrediction,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    aptx_invert_quantization(invert_quantize, quantized_sample, dither, tables);

    let sign = diffsign(invert_quantize.reconstructed_difference, -prediction.predicted_difference);
    let same_sign = [sign * prediction.prev_sign[0], sign * prediction.prev_sign[1]];
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
