const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[derive(Clone, Copy)]
enum Channels {
    LEFT,
    RIGHT,
    NB_CHANNELS,
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
    invert_quantize: [AptxInvertQuantize NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }

    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; Channels::NB_CHANNELS as usize], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[Channels::LEFT as usize])
        ^ aptx_quantized_parity(&channels[Channels::RIGHT as usize]);
    let eighth = *sync_idx == 7;

    *sync_idx = (*sync_idx).wrapping_add(1) & 7;
    if parity ^ (eighth as i32) != 0 {
        1
    } else {
        0
    }
}

fn aptx_insert_sync(channels: &mut [AptxChannel; Channels::NB_CHANNELS as usize], sync_idx: &mut u8) {
    let map: [usize; 4] = [1, 2, 0, 3];

    if aptx_check_parity(channels, sync_idx) != 0 {
        let mut min_error = i32::MAX;
        let mut min_index: Option<(usize, usize)> = None;

        for (c_idx, c) in channels.iter().enumerate() {
            for i in 0..NB_SUBBANDS {
                if c.quantize[map[i]].error < min_error {
                    min_error = c.quantize[map[i]].error;
                    min_index = Some((c_idx, map[i]));
                }
            }
        }

        if let Some((c_idx, subband_idx)) = min_index {
            channels[c_idx].quantize[subband_idx].quantized_sample = channels[c_idx].quantize[subband_idx].quantized_sample_parity_change;
        }
    }
}
