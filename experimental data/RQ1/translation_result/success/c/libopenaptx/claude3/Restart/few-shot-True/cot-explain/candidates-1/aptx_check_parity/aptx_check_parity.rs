
use std::boxed::Box;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[derive(Clone, Copy)]
enum Channels {
    Left,
    Right,
}

const NB_CHANNELS: usize = 2;

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

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    
    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; NB_CHANNELS], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[Channels::Left as usize])
        ^ aptx_quantized_parity(&channels[Channels::Right as usize]);
    let eighth = (*sync_idx == 7) as i32;
    
    *sync_idx = (*sync_idx as usize + 1) as u8 & 7;
    parity ^ eighth
}
