#[derive(Debug)]
struct aptx_filter_signal {
    buffer: [i32; 32],
    pos: u8,
}

#[derive(Debug)]
struct aptx_prediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Debug)]
struct aptx_invert_quantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Debug)]
struct aptx_quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Debug)]
struct aptx_QMF_analysis {
    outer_filter_signal: [aptx_filter_signal; 2],
    inner_filter_signal: [[aptx_filter_signal; 2]; 2],
}

#[derive(Debug)]
struct aptx_channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 4],
    qmf: aptx_QMF_analysis,
    quantize: [aptx_quantize; 4],
    invert_quantize: [aptx_invert_quantize; 4],
    prediction: [aptx_prediction; 4],
}

fn aptx_update_codeword_history(channel: &mut aptx_channel) {
    let cw = ((channel.quantize[0].quantized_sample & 3) << 0) +
             ((channel.quantize[1].quantized_sample & 2) << 1) +
             ((channel.quantize[2].quantized_sample & 1) << 3);
    channel.codeword_history = (cw << 8).wrapping_add((channel.codeword_history as u32).wrapping_shl(4) as i32);
}
