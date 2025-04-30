
const FILTER_TAPS: usize = 16;

#[derive(Debug)]
pub struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

pub fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    let pos = signal.pos as usize;
    signal.buffer[pos] = sample;
    signal.buffer[pos + FILTER_TAPS] = sample;
    signal.pos = (signal.pos.wrapping_add(1) % FILTER_TAPS as u8) as u8;
}
