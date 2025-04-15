#[allow(dead_code)]
const NB_FILTERS: usize = 2;
#[allow(dead_code)]
const NB_SUBBANDS: usize = 4;
#[allow(dead_code)]
const FILTER_TAPS: usize = 16;

#[allow(dead_code)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[allow(dead_code)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[allow(dead_code)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[allow(dead_code)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

#[allow(dead_code)]
fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }

    parity & 1
}

#[allow(dead_code)]
fn aptx_pack_codeword(channel: &AptxChannel) -> u16 {
    let parity = aptx_quantized_parity(channel);
    let q3 = channel.quantize[3].quantized_sample;
    let q2 = channel.quantize[2].quantized_sample;
    let q1 = channel.quantize[1].quantized_sample;
    let q0 = channel.quantize[0].quantized_sample;

    ((((q3 & 0x06) | parity) << 13) |
     ((q2 & 0x03) << 11) |
     ((q1 & 0x0F) << 7) |
     ((q0 & 0x7F) << 0)) as u16
}
