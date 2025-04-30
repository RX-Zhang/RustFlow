use std::mem::MaybeUninit;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos as usize).wrapping_add(FILTER_TAPS)] = sample;
    signal.pos = (signal.pos.wrapping_add(1) & (FILTER_TAPS as u8 - 1)) as u8
}

impl AptxFilterSignal {
    fn new() -> Self {
        let mut buffer: [MaybeUninit<i32>; 2 * FILTER_TAPS] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for elem in &mut buffer[..] {
            *elem = MaybeUninit::new(0);
        }
        Self {
            buffer: unsafe { std::mem::transmute(buffer) },
            pos: 0,
        }
    }
}
