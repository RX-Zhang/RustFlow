use std::mem::size_of_val;

#[derive(Clone, Copy)]
enum Channels {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct AptxFilterSignal {
    buffer: [i32; 32],
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
struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; 2],
    inner_filter_signal: [[AptxFilterSignal; 2]; 2],
}

#[derive(Clone, Copy)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 4],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; 4],
    invert_quantize: [AptxInvertQuantize; 4],
    prediction: [AptxPrediction; 4],
}

#[derive(Clone, Copy)]
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

const LATENCY_SAMPLES: usize = 90;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

fn aptx_reset(ctx: &mut Box<AptxContext>) {
    let hd = ctx.hd;
    *ctx = Box::new(AptxContext::default()); // safer reset using default
    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    for chan in 0..2 {
        for subband in 0..4 {
            ctx.channels[chan].prediction[subband].prev_sign[0] = 1;
            ctx.channels[chan].prediction[subband].prev_sign[1] = 1;
        }
    }
}

fn aptx_reset_decode_sync(ctx: &mut Box<AptxContext>) {
    let decode_dropped = ctx.decode_dropped;
    let decode_sync_packets = ctx.decode_sync_packets;
    let decode_sync_buffer_len = ctx.decode_sync_buffer_len;
    let decode_sync_buffer = ctx.decode_sync_buffer;

    aptx_reset(ctx);

    ctx.decode_sync_buffer = decode_sync_buffer;
    ctx.decode_sync_buffer_len = decode_sync_buffer_len;
    ctx.decode_sync_packets = decode_sync_packets;
    ctx.decode_dropped = decode_dropped;
}

impl Default for AptxContext {
    fn default() -> Self {
        AptxContext {
            decode_sync_packets: 0,
            decode_dropped: 0,
            channels: [AptxChannel::default(); 2],
            hd: 0,
            sync_idx: 0,
            encode_remaining: 0,
            decode_skip_leading: 0,
            decode_sync_buffer_len: 0,
            decode_sync_buffer: [0u8; 6],
        }
    }
}

impl Default for AptxChannel {
    fn default() -> Self {
        AptxChannel {
            codeword_history: 0,
            dither_parity: 0,
            dither: [0; 4],
            qmf: AptxQmfAnalysis::default(),
            quantize: [AptxQuantize::default(); 4],
            invert_quantize: [AptxInvertQuantize::default(); 4],
            prediction: [AptxPrediction::default(); 4],
        }
    }
}

impl Default for AptxQmfAnalysis {
    fn default() -> Self {
        AptxQmfAnalysis {
            outer_filter_signal: [AptxFilterSignal::default(); 2],
            inner_filter_signal: [[AptxFilterSignal::default(); 2]; 2],
        }
    }
}

impl Default for AptxFilterSignal {
    fn default() -> Self {
        AptxFilterSignal {
            buffer: [0; 32],
            pos: 0,
        }
    }
}

impl Default for AptxPrediction {
    fn default() -> Self {
        AptxPrediction {
            prev_sign: [1; 2], //Initialized to 1 as per original code.
            s_weight: [0; 2],
            d_weight: [0; 24],
            pos: 0,
            reconstructed_differences: [0; 48],
            previous_reconstructed_sample: 0,
            predicted_difference: 0,
            predicted_sample: 0,
        }
    }
}

impl Default for AptxInvertQuantize {
    fn default() -> Self {
        AptxInvertQuantize {
            quantization_factor: 0,
            factor_select: 0,
            reconstructed_difference: 0,
        }
    }
}

impl Default for AptxQuantize {
    fn default() -> Self {
        AptxQuantize {
            quantized_sample: 0,
            quantized_sample_parity_change: 0,
            error: 0,
        }
    }
}
