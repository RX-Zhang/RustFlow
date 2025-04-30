

use std::mem;

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: usize,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    return ((value + rounding) >> shift) - (if (value & mask) == rounding { 1 } else { 0 });
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos..];
    let mut e: i64 = 0;
    for i in 0..FILTER_TAPS {
        e += sig[i] as i64 * coeffs[i] as i64;
    }
    return rshift64_clip24(e, shift);
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos] = sample;
    signal.buffer[signal.pos + FILTER_TAPS] = sample;
    signal.pos = (signal.pos + 1) & (FILTER_TAPS - 1);
}

fn aptx_qmf_polyphase_synthesis(
    signal: &mut [AptxFilterSignal],
    coeffs: &[[i32; FILTER_TAPS]],
    shift: u32,
    low_subband_input: i32,
    high_subband_input: i32,
    samples: &mut [i32],
) {
    let mut subbands: [i32; NB_FILTERS] = [0; NB_FILTERS];
    subbands[0] = low_subband_input + high_subband_input;
    subbands[1] = low_subband_input - high_subband_input;

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], subbands[1 - i]);
        samples[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }
}

