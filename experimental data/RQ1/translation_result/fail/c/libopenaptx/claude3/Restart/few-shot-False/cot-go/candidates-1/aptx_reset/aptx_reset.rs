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

#[derive(Clone, Default)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Clone, Default)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Clone, Default)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Clone, Default)]
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

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut new_ctx = AptxContext {
        decode_sync_packets: 0,
        decode_dropped: 0,
        channels: Default::default(),
        hd,
        sync_idx: 0,
        encode_remaining: ((LATENCY_SAMPLES.wrapping_add(3)) / 4) as u8,
        decode_skip_leading: ((LATENCY_SAMPLES.wrapping_add(3)) / 4) as u8,
        decode_sync_buffer_len: 0,
        decode_sync_buffer: [0; 6],
    };

    for chan in 0..NB_CHANNELS {
        for subband in 0..NB_SUBBANDS {
            new_ctx.channels[chan].prediction[subband].prev_sign[0] = 1;
            new_ctx.channels[chan].prediction[subband].prev_sign[1] = 1;
        }
    }

    mem::swap(ctx, &mut new_ctx);
}
