use std::boxed::Box;

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &Box<AptxFilterSignal>, coeffs: &[i32; 16], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..];
    let mut e: i64 = 0;

    for i in 0..16 {
        e = e.wrapping_add(sig[i] as i64 * coeffs[i] as i64);
    }

    rshift64_clip24(e, shift)
}

struct AptxFilterSignal {
    buffer: [i32; 32],
    pos: u8,
}
