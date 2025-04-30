const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    let pos = signal.pos as usize;
    signal.buffer[pos] = sample;
    signal.buffer[pos + FILTER_TAPS] = sample;
    signal.pos = (signal.pos.wrapping_add(1)).wrapping_rem(FILTER_TAPS as u8);
}
