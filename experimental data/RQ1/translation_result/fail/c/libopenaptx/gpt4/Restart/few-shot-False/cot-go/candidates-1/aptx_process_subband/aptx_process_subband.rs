fn aptx_prediction_filtering(prediction: &mut AptxPrediction, reconstructed_difference: i32, order: i32) {
    let mut reconstructed_sample = clip_intp2(
        reconstructed_difference.wrapping_add(prediction.predicted_sample),
        23,
    );

    let predictor = clip_intp2(
        (((prediction.s_weight[0] as i64 * prediction.previous_reconstructed_sample as i64)
            + (prediction.s_weight[1] as i64 * reconstructed_sample as i64))
            >> 22) as i32,
        23,
    );

    prediction.previous_reconstructed_sample = reconstructed_sample;

    let reconstructed_differences =
        aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);

    let srd0 = diffsign(reconstructed_difference, 0) * (1 << 23);
    let mut predicted_difference = 0i64;

    for i in 0..order as usize {
        let srd = (prediction.reconstructed_differences[i].wrapping_shr(31)) | 1;
        prediction.d_weight[i] = prediction.d_weight[i].wrapping_sub(
            rshift32(prediction.d_weight[i].wrapping_sub(srd * srd0), 8),
        );
        predicted_difference +=
            (prediction.reconstructed_differences[i] as i64 * prediction.d_weight[i] as i64);
    }

    prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23)
    prediction.predicted_sample =
        clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}
