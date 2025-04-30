

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

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    let v = (val as u32).wrapping_shl(shift);
    (v as i32).wrapping_shr(shift)
}

fn aptx_unpack_codeword(channel: &mut AptxChannel, codeword: u16) {
    channel.quantize[0].quantized_sample = sign_extend((codeword >> 0) as i32, 7);
    channel.quantize[1].quantized_sample = sign_extend((codeword >> 7) as i32, 4);
    channel.quantize[2].quantized_sample = sign_extend((codeword >> 11) as i32, 2);
    channel.quantize[3].quantized_sample = sign_extend((codeword >> 13) as i32, 3);
    channel.quantize[3].quantized_sample = (channel.quantize[3].quantized_sample & !1)
        | aptx_quantized_parity(channel);
}
