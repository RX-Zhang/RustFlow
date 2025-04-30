use std::mem;

#[derive(Clone, Copy)]
struct AptxFilterSignal {
 buffer: [i32; 32],
 pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
 if ((a as u32).wrapping_add((1u32).wrapping_shl(p))) & (!((2u32).wrapping_shl(p)).wrapping_sub(1)) != 0 {
  (a >> 31) ^ (((1 as i32) << p) - 1)
 } else {
  a
 }
}

fn rshift64(value: i64, shift: u32) -> i64 {
 let rounding: i64 = 1 << (shift - 1);
 let mask: i64 = ((1 as i64) << (shift + 1)) - 1;
 ((value).wrapping_add(rounding) >> shift) - (if (value & mask) == rounding { 1 } else { 0 })
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
 clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; 16], shift: u32) -> i32 {
 let sig = &signal.buffer[signal.pos as usize..];
 let mut e: i64 = 0;
 
 for i in 0..16 {
  e = e.wrapping_add((sig[i] as i64).wrapping_mul(coeffs[i] as i64));
 }

 rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
 signal.buffer[signal.pos as usize] = sample;
 signal.buffer[(signal.pos as usize) + 16] = sample;
 signal.pos = (signal.pos.wrapping_add(1) & 15) as u8;
}

fn aptx_qmf_polyphase_analysis(
 signal: &mut [AptxFilterSignal; 2],
 coeffs: &[[i32; 16]; 2],
 shift: u32,
 samples: &[i32; 2],
 low_subband_output: &mut i32,
 high_subband_output: &mut i32,
) {
 let mut subbands: [i32; 2] = [0, 0];

 for i in 0..2 {
  aptx_qmf_filter_signal_push(&mut signal[i], samples[1 - i]);
  subbands[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
 }

 *low_subband_output = clip_intp2(subbands[0].wrapping_add(subbands[1]), 23);
 *high_subband_output = clip_intp2(subbands[0].wrapping_sub(subbands[1]), 23);
}
