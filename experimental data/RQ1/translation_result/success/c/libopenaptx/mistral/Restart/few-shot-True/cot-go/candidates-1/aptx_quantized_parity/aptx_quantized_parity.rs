use std::mem::MaybeUninit;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[derive(Clone, Copy)]
struct aptx_filter_signal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct aptx_invert_quantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Clone, Copy)]
struct aptx_quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Clone, Copy)]
struct aptx_QMF_analysis {
    outer_filter_signal: [aptx_filter_signal; NB_FILTERS],
    inner_filter_signal: [[aptx_filter_signal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Clone, Copy)]
struct aptx_channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: aptx_QMF_analysis,
    quantize: [aptx_quantize; NB_SUBBANDS],
    invert_quantize: [aptx_invert_quantize; NB_SUBBANDS],
    prediction: [aptx_prediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &aptx_channel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}
