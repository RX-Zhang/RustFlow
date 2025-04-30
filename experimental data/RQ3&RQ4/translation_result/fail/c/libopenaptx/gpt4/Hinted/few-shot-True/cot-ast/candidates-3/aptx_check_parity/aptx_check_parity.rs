const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;
const NB_CHANNELS: usize = 2;

#[derive(Debug, Clone)]
pub enum Channels {
    Left = 0,
    Right = 1,
    NbChannels = 2,
}

#[derive(Debug, Clone)]
pub struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Debug, Clone)]
pub struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Debug, Clone)]
pub struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Debug, Clone)]
pub struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Debug, Clone)]
pub struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Debug, Clone)]
pub struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

pub fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity = parity ^ channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

pub fn aptx_check_parity(channels: &[AptxChannel; NB_CHANNELS], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[Channels::Left as usize])
        ^ aptx_quantized_parity(&channels[Channels::Right as usize]);
    let eighth = if *sync_idx == 7 { 1 } else { 0 };
    *sync_idx = (*sync_idx).wrapping_add(1) & 7;
    parity ^ eighth
}
