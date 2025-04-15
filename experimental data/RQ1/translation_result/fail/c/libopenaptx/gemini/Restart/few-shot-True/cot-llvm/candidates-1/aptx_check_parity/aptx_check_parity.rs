use std::boxed::Box;

enum Channels {
    Left,
    Right,
}

struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
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
    inner_filter_signal: [[AptxFilterSignal; 2]; 2],
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

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for i in 0..4 {
        parity ^= channel.quantize[i].quantized_sample;
    }
    parity & 1
}

fn aptx_check_parity(channels: &[Box<AptxChannel>; 2], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[Channels::Left as usize])
        ^ aptx_quantized_parity(&channels[Channels::Right as usize]);
    let eighth = *sync_idx == 7;
    *sync_idx = (*sync_idx + 1) % 8;
    parity ^ eighth as i32
}
