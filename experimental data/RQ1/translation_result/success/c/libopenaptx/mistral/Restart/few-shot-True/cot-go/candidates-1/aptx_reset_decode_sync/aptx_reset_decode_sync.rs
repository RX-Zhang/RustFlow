use std::mem::size_of;

const LATENCY_SAMPLES: usize = 90;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

enum Channels {
 Left,
 Right,
}

struct AptxFilterSignal {
 buffer: [i32; 2 * FILTER_TAPS],
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
 outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
 inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

struct AptxChannel {
 codeword_history: i32,
 dither_parity: i32,
 dither: [i32; NB_SUBBANDS],
 qmf: AptxQmfAnalysis,
 quantize: [AptxQuantize; NB_SUBBANDS],
 invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
 prediction: [AptxPrediction; NB_SUBBANDS],
}

struct AptxContext {
 decode_sync_packets: usize,
 decode_dropped: usize,
 channels: [AptxChannel; 2], //NB_CHANNELS
 hd: u8,
 sync_idx: u8,
 encode_remaining: u8,
 decode_skip_leading: u8,
 decode_sync_buffer_len: u8,
 decode_sync_buffer: [u8; 6],
}

fn aptx_reset(ctx: &mut Box<AptxContext>) {
 let hd = ctx.hd;
 
 unsafe {
  std::ptr::write_bytes(&mut **ctx as *mut AptxContext as *mut u8, 0, size_of::<AptxContext>());
 }

 ctx.hd = hd;
 ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
 ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

 for chan_idx in 0..2 { //NB_CHANNELS
  for subband in 0..NB_SUBBANDS {
  ctx.channels[chan_idx].prediction[subband].prev_sign[0] = 1;
  ctx.channels[chan_idx].prediction[subband].prev_sign[1] = 1;
  }
 }
}

fn aptx_reset_decode_sync(ctx: &mut Box<AptxContext>) {
 let decode_dropped = ctx.decode_dropped;
 let decode_sync_packets = ctx.decode_sync_packets;
 let decode_sync_buffer_len = ctx.decode_sync_buffer_len;
 let decode_sync_buffer = ctx.decode_sync_buffer;

 aptx_reset(ctx);

 ctx.decode_sync_buffer = decode_sync_buffer;
 ctx.decode_sync_buffer_len = decode_sync_buffer_len;
 ctx.decode_sync_packets = decode_sync_packets;
 ctx.decode_dropped = decode_dropped;
}
