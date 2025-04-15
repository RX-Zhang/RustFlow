use std::convert::TryInto;

fn clip_intp2(a: i32, p: u32) -> i32 {
 if ((a as u32).wrapping_add((1u32 << p))) & !((((1u32) << p) * 2).wrapping_sub(1)) != 0 {
  (a >> 31) ^ (((1 << p) as i32).wrapping_sub(1))
 } else {
  a
 }
}

fn rshift64(value: i64, shift: u32) -> i64 {
 let rounding: i64 = (1 as i64) << (shift - 1);
 let mask: i64 = (((1 as i64) << shift).wrapping_sub(1));
 ((value.wrapping_add(rounding)) >> shift) - (((value & mask) == rounding) as i64)
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

fn aptx_qmf_filter_signal_push(signal: &mut Box<AptxFilterSignal>, sample: i32) {
 signal.buffer[signal.pos as usize] = sample;
 signal.buffer[((signal.pos as i32).wrapping_add(16)) as usize] = sample;
 signal.pos = (signal.pos + 1) % 16;
}

fn aptx_qmf_polyphase_analysis(signal: &mut [Box<AptxFilterSignal>; 2],
      coeffs: &[[i32; 16]; 2],
      shift: u32,
      samples: &[i32; 2],
      low_subband_output: &mut i32,
      high_subband_output: &mut i32) {
 let mut subbands: [i32; 2] = [0; 2];
 for i in 0..2 {
  aptx_qmf_filter_signal_push(&mut signal[i], samples[1 - i]);
  subbands[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
 }

 *low_subband_output = clip_intp2(subbands[0].wrapping_add(subbands[1]), 23);
 *high_subband_output = clip_intp2(subbands[0].wrapping_sub(subbands[1]), 23);
}


struct AptxFilterSignal {
 buffer: [i32; 32],
 pos: u8,
}
