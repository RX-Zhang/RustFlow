use std::boxed::Box;

struct AptxFilterSignal {
    buffer: [i32; 32],
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

struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; 2],
    inner_filter_signal: [[AptxFilterSignal; 4]; 2],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 4],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; 4],
    invert_quantize: [AptxInvertQuantize; 4],
    prediction: [AptxPrediction; 4],
}

fn aptx_quantized_parity(channel: &Box<AptxChannel>) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..4 {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}
