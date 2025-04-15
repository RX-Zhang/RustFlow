

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

struct aptx_filter_signal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

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

struct aptx_invert_quantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

struct aptx_quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct aptx_QMF_analysis {
    outer_filter_signal: [aptx_filter_signal; NB_FILTERS],
    inner_filter_signal: [[aptx_filter_signal; NB_FILTERS]; NB_FILTERS],
}

struct aptx_channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: aptx_QMF_analysis,
    quantize: [aptx_quantize; NB_SUBBANDS],
    invert_quantize: [aptx_invert_quantize; NB_SUBBANDS],
    prediction: [aptx_prediction; NB_SUBBANDS],
}

fn aptx_update_codeword_history(channel: &mut aptx_channel) {
    let cw = ((channel.quantize[0].quantized_sample & 3) << 0) +
        ((channel.quantize[1].quantized_sample & 2) << 1) +
        ((channel.quantize[2].quantized_sample & 1) << 3);
    channel.codeword_history = (cw << 8).wrapping_add((channel.codeword_history as u32).wrapping_shl(4) as i32);
}
