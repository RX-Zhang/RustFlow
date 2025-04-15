use std::mem::size_of;

#[derive(Clone, Copy)]
enum Channels {
 Left,
 Right,
}

struct AptxFilterSignal {
 buffer: [i32; 2 * 16],
 pos: u8,
}

struct AptxPrediction {
 prev_sign: [i32; 2],
 s_weight: [i32; 2],
 d_weight: [i32; 24],
 pos: i32,
 reconstructed_differences: [i32; 48],
 previous_reconstructed_sample: i32,
 predicted_difference: i32,
 predicted_sample: i32,
}

struct AptxInvertQuantize {
 quantization_factor: i32,
 factor_select: i32,
 reconstructed_difference: i32,
}

struct AptxQuantize {
 quantized_sample: i32,
 quantized_sample_parity_change: i32,
 error: i32,
}

struct AptxQmfAnalysis {
 outer_filter_signal: [AptxFilterSignal; 2],
 inner_filter_signal: [[AptxFilterSignal; 2]; 2],
}

struct AptxChannel {
 codeword_history: i32,
 dither_parity: i32,
 dither: [i32; 4],
 qmf: AptxQmfAnalysis,
 quantize: [AptxQuantize; 4],
 invert_quantize: [AptxInvertQuantize; 4],
 prediction: [AptxPrediction; 4],
}

struct AptxContext {
 decode_sync_packets: usize,
 decode_dropped: usize,
 channels: [AptxChannel; 2],
 hd: u8,
 sync_idx: u8,
 encode_remaining: u8,
 decode_skip_leading: u8,
 decode_sync_buffer_len: u8,
 decode_sync_buffer: [u8; 6],
}

fn aptx_reset(ctx: &mut AptxContext) {
 let hd = ctx.hd;
 let size = size_of::<AptxContext>();

 let ptr = ctx as *mut AptxContext as *mut u8;
 let slice = unsafe { std::slice::from_raw_parts_mut(ptr, size) };
 for byte in slice.iter_mut() {
  *byte = 0;
 }


 ctx.hd = hd;
 ctx.decode_skip_leading = ((90 + 3) / 4) as u8;
 ctx.encode_remaining = ((90 + 3) / 4) as u8;

 for chan in 0..2 {
  for subband in 0..4 {
  ctx.channels[chan].prediction[subband].prev_sign[0] = 1;
  ctx.channels[chan].prediction[subband].prev_sign[1] = 1;
  }
 }
}
