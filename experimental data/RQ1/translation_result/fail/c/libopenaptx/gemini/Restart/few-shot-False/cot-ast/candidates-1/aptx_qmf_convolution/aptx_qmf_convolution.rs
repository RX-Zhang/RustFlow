const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (((a as u32).wrapping_add((1u32).wrapping_shl(p))) & (!((2u32).wrapping_shl(p).wrapping_sub(1)))) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = 1 << (shift - 1);
    let mask: i64 = ((1i64) << (shift + 1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)).wrapping_shr(shift as u32)).wrapping_sub(if (value & mask) == rounding { 1 } else { 0 })
}

#[inline]
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

#[inline]
fn aptx_qmf_convolution(
    signal: &AptxFilterSignal,
    coeffs: &[i32; FILTER_TAPS],
    shift: u32,
) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..];
    let mut e: i64 = 0;

    for i in 0..FILTER_TAPS {
        e = e.wrapping_add((sig[i] as i64).wrapping_mul(coeffs[i] as i64));
    }

    rshift64_clip24(e, shift)
}
