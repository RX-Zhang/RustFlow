const FILTER_TAPS: usize = 64; // Define FILTER_TAPS according to the expected size

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    let pos = signal.pos as usize;
    signal.buffer[pos] = sample;
    signal.buffer[pos + FILTER_TAPS] = sample;
    signal.pos = ((signal.pos as usize + 1) % FILTER_TAPS) as u8;
}
