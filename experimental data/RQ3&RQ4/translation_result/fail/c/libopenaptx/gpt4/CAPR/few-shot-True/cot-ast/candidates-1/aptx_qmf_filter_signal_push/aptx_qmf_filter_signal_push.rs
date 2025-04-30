struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,


const FILTER_TAPS: usize = 16;

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos as usize + FILTER_TAPS) % (2 * FILTER_TAPS)] = sample;
    signal.pos = ((signal.pos as usize + 1) & (FILTER_TAPS - 1)) as u8;
}
