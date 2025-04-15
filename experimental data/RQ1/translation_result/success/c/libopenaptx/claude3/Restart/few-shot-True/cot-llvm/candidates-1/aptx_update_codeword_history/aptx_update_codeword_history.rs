use std::boxed::Box;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

struct AptxPrediction {
    prev_sign: Box<[i32; 2]>,
    s_weight: Box<[i32; 2]>,
    d_weight: Box<[i32; 24]>,
    pos: i32,
    reconstructed_differences: Box<[i32; 48]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct AptxQMFAnalysis {
    outer_filter_signal: Box<[AptxFilterSignal; NB_FILTERS]>,
    inner_filter_signal: Box<[[AptxFilterSignal; NB_FILTERS]; NB_FILTERS]>,
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: Box<[i32; NB_SUBBANDS]>,
    qmf: AptxQMFAnalysis,
    quantize: Box<[AptxQuantize; NB_SUBBANDS]>,
    invert_quantize: Box<[AptxInvertQuantize; NB_SUBBANDS]>,
    prediction: Box<[AptxPrediction; NB_SUBBANDS]>,
}

fn aptx_update_codeword_history(channel: &mut Box<AptxChannel>) {
    let cw = ((channel.quantize[0].quantized_sample & 3) << 0)
        .wrapping_add((channel.quantize[1].quantized_sample & 2) << 1)
        .wrapping_add((channel.quantize[2].quantized_sample & 1) << 3);
    
    channel.codeword_history = (cw << 8).wrapping_add(((channel.codeword_history as u32 % (1 << 28)) << 4) as i32);
}
