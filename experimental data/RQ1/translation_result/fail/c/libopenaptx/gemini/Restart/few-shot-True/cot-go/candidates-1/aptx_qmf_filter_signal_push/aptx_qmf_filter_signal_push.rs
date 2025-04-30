use std::mem;

#[repr(C)]
pub struct aptx_filter_signal {
    pub buffer: [i32; 32],
    pub pos: u8,
}

pub fn aptx_qmf_filter_signal_push(signal: &mut Box<aptx_filter_signal>, sample: i32) {
    let pos = signal.pos as usize;
    signal.buffer[pos] = sample;
    signal.buffer[pos + 16] = sample;
    signal.pos = ((signal.pos as u16 + 1) & (16 - 1)) as u8;
}
