

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
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

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;

    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }

    parity & 1
}

fn aptx_pack_codeword(channel: &AptxChannel) -> u16 {
    let parity = aptx_quantized_parity(channel);
    
    (((channel.quantize[3].quantized_sample & 0x06) | parity) as u16) << 13
        | ((channel.quantize[2].quantized_sample & 0x03) as u16) << 11
        | ((channel.quantize[1].quantized_sample & 0x0F) as u16) << 7
        | ((channel.quantize[0].quantized_sample & 0x7F) as u16)
}
