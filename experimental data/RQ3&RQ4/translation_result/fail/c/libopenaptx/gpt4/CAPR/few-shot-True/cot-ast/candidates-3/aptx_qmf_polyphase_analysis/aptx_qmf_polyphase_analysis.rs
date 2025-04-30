const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..];
    let mut e: i64 = 0;

    for i in 0..FILTER_TAPS {
        e = e.wrapping_add((sig[i] as i64).wrapping_mul(coeffs[i] as i64));
    }

    rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos as usize) + FILTER_TAPS] = sample;
    signal.pos = (signal.pos + 1) & (FILTER_TAPS - 1) as u8;
}

fn aptx_qmf_polyphase_analysis(
    signal: &mut [AptxFilterSignal; NB_FILTERS],
    coeffs: &[[i32; FILTER_TAPS]; NB_FILTERS],
    shift: u32,
    samples: &[i32; NB_FILTERS],
    low_subband_output: &mut i32,
    high_subband_output: &mut i32,
) {
    let mut subbands = [0i32; NB_FILTERS];

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], samples[NB_FILTERS - 1 - i]);
        subbands[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }

    *low_subband_output = clip_intp2(subbands[0].wrapping_add(subbands[1]), 23);
    *high_subband_output = clip_intp2(subbands[0].wrapping_sub(subbands[1]), 23);
}
