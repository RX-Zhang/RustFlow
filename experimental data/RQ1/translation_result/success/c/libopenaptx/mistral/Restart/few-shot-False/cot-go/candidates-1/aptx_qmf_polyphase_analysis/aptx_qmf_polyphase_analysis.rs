

use libc::{int64_t, int32_t, c_int, uint8_t};
use std::mem;
use std::boxed;

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: boxed::Box<[int32_t; 2*FILTER_TAPS]>,
    pos: uint8_t,
}

fn clip_intp2(a: int32_t, p: u32) -> int32_t {
    if ((a as c_int).wrapping_add(1 << p) & !((1 << (p + 1)) - 1)) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift64(value: int64_t, shift: u32) -> int64_t {
    let rounding: int64_t = (1 << (shift - 1));
    let mask: int64_t = ((1 << (shift + 1)) - 1);
    return ((value.wrapping_add(rounding)) >> shift).wrapping_sub((value & mask).wrapping_sub(rounding));
}

fn rshift64_clip24(value: int64_t, shift: u32) -> int32_t {
    return clip_intp2(value as int32_t, 23);
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[int32_t], shift: u32) -> int32_t {
    let sig: &[int32_t] = &signal.buffer[..FILTER_TAPS].to_vec()[signal.pos as usize..][..];
    let mut e: int64_t = 0;
    let mut i: usize = 0;

    for i in 0..FILTER_TAPS {
        e = e.wrapping_add((sig[i] as int64_t) * (coeffs[i] as int64_t));
    }

    return rshift64_clip24(e, shift);
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: int32_t) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos as usize) + FILTER_TAPS] = sample;
    signal.pos = (signal.pos.wrapping_add(1)) & (FILTER_TAPS as u8 - 1);
}

fn aptx_qmf_polyphase_analysis(signal: &mut [AptxFilterSignal],
                               coeffs: &[[int32_t; FILTER_TAPS]],
                               shift: u32,
                               samples: &[int32_t],
                               low_subband_output: &mut int32_t,
                               high_subband_output: &mut int32_t) {
    let mut subbands: [int32_t; NB_FILTERS] = [0; NB_FILTERS];
    let mut i: usize = 0;

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], samples[NB_FILTERS - 1 - i]);
        subbands[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }

    unsafe {
        *low_subband_output = clip_intp2(subbands[0].wrapping_add(subbands[1]), 23);
        *high_subband_output = clip_intp2(subbands[0].wrapping_sub(subbands[1]), 23);
    }
}

