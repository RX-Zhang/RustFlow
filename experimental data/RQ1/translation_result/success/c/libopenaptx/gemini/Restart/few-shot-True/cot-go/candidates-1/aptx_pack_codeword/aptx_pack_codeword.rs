use std::convert::TryInto;

const NB_FILTERS: i32 = 2;
const NB_SUBBANDS: i32 = 4;
const FILTER_TAPS: i32 = 16;

#[derive(Copy, Clone)]
struct aptx_filter_signal {
    buffer: [i32; 2 * FILTER_TAPS as usize],
    pos: u8,
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
struct aptx_invert_quantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Copy, Clone)]
struct aptx_quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Copy, Clone)]
struct aptx_QMF_analysis {
    outer_filter_signal: [aptx_filter_signal; NB_FILTERS as usize],
    inner_filter_signal: [[aptx_filter_signal; NB_FILTERS as usize]; NB_FILTERS as usize],
}

#[derive(Copy, Clone)]
struct aptx_channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS as usize],
    qmf: aptx_QMF_analysis,
    quantize: [aptx_quantize; NB_SUBBANDS as usize],
    invert_quantize: [aptx_invert_quantize; NB_SUBBANDS as usize],
    prediction: [aptx_prediction; NB_SUBBANDS as usize],
}

fn aptx_quantized_parity(channel: &aptx_channel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS as usize {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn aptx_pack_codeword(channel: &aptx_channel) -> u16 {
    let parity = aptx_quantized_parity(channel);
    let value: u32 = ((((channel.quantize[3].quantized_sample & 0x06) | parity) as u32) << 13)
        .wrapping_add((((channel.quantize[2].quantized_sample & 0x03) as u32) << 11))
        .wrapping_add((((channel.quantize[1].quantized_sample & 0x0F) as u32) << 7))
        .wrapping_add(((channel.quantize[0].quantized_sample & 0x7F) as u32) << 0);
    value.try_into().unwrap()
}
