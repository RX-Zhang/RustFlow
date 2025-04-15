const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;
const NB_CHANNELS: usize = 2;
const LEFT: usize = 0;
const RIGHT: usize = 1;

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

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    let mut subband: usize = 0;

    while subband < NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
        subband = subband.wrapping_add(1);
    }

    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; NB_CHANNELS], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[LEFT]) ^ aptx_quantized_parity(&channels[RIGHT]);
    let eighth = ((*sync_idx == 7) as i32) & 1;

    *sync_idx = (*sync_idx).wrapping_add(1) & 7;
    parity ^ eighth
}

fn aptx_insert_sync(channels: &mut [AptxChannel; NB_CHANNELS], sync_idx: &mut u8) {
    let map: [usize; NB_SUBBANDS] = [1, 2, 0, 3];

    if aptx_check_parity(channels, sync_idx) != 0 {
        // Initialize with the first element in the map
        let mut min_error = channels[NB_CHANNELS - 1].quantize[map[0]].error;
        let mut min_channel = NB_CHANNELS - 1;
        let mut min_subband = map[0];

        // Iterate over channels and subbands to find the minimum error
        for (c_idx, c) in channels.iter().enumerate().rev() {
            for &i in &map {
                if c.quantize[i].error < min_error {
                    min_error = c.quantize[i].error;
                    min_channel = c_idx;
                    min_subband = i;
                }
            }
        }

        // Mutably borrow the specific quantize element and modify it
        channels[min_channel].quantize[min_subband].quantized_sample =
            channels[min_channel].quantize[min_subband].quantized_sample_parity_change;
    }
}
