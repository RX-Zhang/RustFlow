use std::boxed::Box;

const LATENCY_SAMPLES: usize = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;

enum Channels {
    Left,
    Right,
}

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

struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [AptxChannel; 2],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    for i in 0..std::mem::size_of::<AptxContext>() {
        unsafe {
            *ctx.as_bytes_mut().get_unchecked_mut(i) = 0;
        }
    }
    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    for chan in 0..2 {
        for subband in 0..NB_SUBBANDS {
            ctx.channels[chan].prediction[subband].prev_sign[0] = 1;
            ctx.channels[chan].prediction[subband].prev_sign[1] = 1;
        }
    }
}

fn aptx_init(hd: i32) -> Box<AptxContext> {
    let hd_bool = hd != 0;
    let ctx: Box<AptxContext> = Box::new(AptxContext{
        decode_sync_packets: 0,
        decode_dropped: 0,
        channels: [AptxChannel{
            codeword_history: 0,
            dither_parity: 0,
            dither: [0; NB_SUBBANDS],
            qmf: AptxQmfAnalysis{
                outer_filter_signal: [AptxFilterSignal{buffer: [0; 2*FILTER_TAPS], pos: 0}; NB_FILTERS],
                inner_filter_signal: [[AptxFilterSignal{buffer: [0; 2*FILTER_TAPS], pos: 0}; NB_FILTERS]; NB_FILTERS]
            },
            quantize: [AptxQuantize{quantized_sample: 0, quantized_sample_parity_change: 0, error: 0}; NB_SUBBANDS],
            invert_quantize: [AptxInvertQuantize{quantization_factor: 0, factor_select: 0, reconstructed_difference: 0}; NB_SUBBANDS],
            prediction: [AptxPrediction{prev_sign: [0; 2], s_weight: [0; 2], d_weight: [0; 24], pos: 0, reconstructed_differences: [0; 48], previous_reconstructed_sample: 0, predicted_difference: 0, predicted_sample: 0}; NB_SUBBANDS]
        }; 2],
        hd: hd_bool as u8,
        sync_idx: 0,
        encode_remaining: 0,
        decode_skip_leading: 0,
        decode_sync_buffer_len: 0,
        decode_sync_buffer: [0; 6],
    });

    aptx_reset(&mut *ctx);
    ctx
}
