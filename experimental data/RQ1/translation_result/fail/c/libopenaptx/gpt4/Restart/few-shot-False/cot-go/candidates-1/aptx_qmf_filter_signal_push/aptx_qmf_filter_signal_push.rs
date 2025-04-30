use std::boxed::Box;

const FILTER_TAPS: usize = 16

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: usize,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos] = sample;
    signal.buffer[signal.pos + FILTER_TAPS] = sample;
    signal.pos = (signal.pos + 1) % FILTER_TAPS;
}
