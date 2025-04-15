const FILTER_TAPS: usize = 16;

#[derive(Default)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    let pos_usize = signal.pos as usize;
    signal.buffer[pos_usize] = sample;
    signal.buffer[pos_usize + FILTER_TAPS] = sample;
    signal.pos = signal.pos.wrapping_add(1) & (FILTER_TAPS as u8 - 1);
}
