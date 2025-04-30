
use std::mem;

const LATENCY_SAMPLES: usize = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const NB_CHANNELS: usize = 2;

#[derive(Clone, Copy)]
enum Channels {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Clone, Copy)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Clone, Copy)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Clone, Copy)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

#[derive(Clone)]
struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [AptxChannel; NB_CHANNELS],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

fn aptx_reset(ctx: &mut Box<AptxContext>) {
    let hd = ctx.hd;
    let mut new_ctx = Box::new(AptxContext {
        decode_sync_packets: 0,
        decode_dropped: 0,
        channels: [AptxChannel {
            codeword_history: 0,
            dither_parity: 0,
            dither: [0; NB_SUBBANDS],
            qmf: AptxQMFAnalysis {
                outer_filter_signal: [AptxFilterSignal { buffer: [0; 2 * FILTER_TAPS], pos: 0 }; NB_FILTERS],
                inner_filter_signal: [[AptxFilterSignal { buffer: [0; 2 * FILTER_TAPS], pos: 0 }; NB_FILTERS]; NB_FILTERS],
            },
            quantize: [AptxQuantize { quantized_sample: 0, quantized_sample_parity_change: 0, error: 0 }; NB_SUBBANDS],
            invert_quantize: [AptxInvertQuantize { quantization_factor: 0, factor_select: 0, reconstructed_difference: 0 }; NB_SUBBANDS],
            prediction: [AptxPrediction {
                prev_sign: [1, 1],
                s_weight: [0; 2],
                d_weight: [0; 24],
                pos: 0,
                reconstructed_differences: [0; 48],
                previous_reconstructed_sample: 0,
                predicted_difference: 0,
                predicted_sample: 0,
            }; NB_SUBBANDS],
        }; NB_CHANNELS],
        hd,
        sync_idx: 0,
        encode_remaining: ((LATENCY_SAMPLES + 3) / 4) as u8,
        decode_skip_leading: ((LATENCY_SAMPLES + 3) / 4) as u8,
        decode_sync_buffer_len: 0,
        decode_sync_buffer: [0; 6],
    });

    mem::swap(ctx, &mut new_ctx);
}
