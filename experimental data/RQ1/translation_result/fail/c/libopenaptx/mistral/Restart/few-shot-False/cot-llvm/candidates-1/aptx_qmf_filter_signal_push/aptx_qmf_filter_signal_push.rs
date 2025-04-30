pub struct AptxFilterSignal {
    buffer: [i32; 32],
    pos: u8,
}

pub fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    let pos = signal.pos as usize;
    signal.buffer[pos] = sample;
    signal.buffer[pos + 16] = sample;
    signal.pos = signal.pos.wrapping_add(1) % 16;
}
