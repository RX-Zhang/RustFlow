use std::cmp::Ordering;

#[derive(Debug)]
pub struct AptxPrediction {
 prev_sign: [i32; 2],
 s_weight: [i32; 2],
 d_weight: [i32; 24],
 pos: i32,
 reconstructed_differences: [i32; 48],
 previous_reconstructed_sample: i32,
 predicted_difference: i32,
 predicted_sample: i32,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
 if ((a as u32).wrapping_add((1u32 << p))) & (!((2u32 << p).wrapping_sub(1))) != 0 {
  (a >> 31) ^ ((1 << p) - 1)
 } else {
  a
 }
}

fn rshift32(value: i32, shift: u32) -> i32 {
 let rounding: i32 = 1 << (shift - 1);
 let mask: i32 = (1 << (shift + 1)) - 1;
 ((value.wrapping_add(rounding)) >> shift).wrapping_sub(if (value & mask) == rounding { 1 } else { 0 })
}

fn aptx_reconstructed_differences_update(
 prediction: &mut AptxPrediction,
 reconstructed_difference: i32,
 order: usize,
) {
 let mut p = prediction.pos as usize;
 let rd1 = prediction.reconstructed_differences[p];

 prediction.reconstructed_differences[(p + order) % prediction.reconstructed_differences.len()] = rd1;
 prediction.pos = ((p + 1) % order) as i32;
 p = prediction.pos as usize;
 prediction.reconstructed_differences[p] = reconstructed_difference;
}

fn aptx_prediction_filtering(
 prediction: &mut AptxPrediction,
 reconstructed_difference: i32,
 order: usize,
) {
 let reconstructed_sample = clip_intp2(reconstructed_difference.wrapping_add(prediction.predicted_sample), 23);
 let predictor = clip_intp2(
  ((prediction.s_weight[0] as i64).wrapping_mul(prediction.previous_reconstructed_sample as i64)
  .wrapping_add((prediction.s_weight[1] as i64).wrapping_mul(reconstructed_sample as i64)) >> 22) as i32,
  23,
 );
 prediction.previous_reconstructed_sample = reconstructed_sample;

 aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
 let reconstructed_differences = &prediction.reconstructed_differences;

 let srd0 = match reconstructed_difference.cmp(&0) {
  Ordering::Greater => 1,
  Ordering::Less => -1,
  Ordering::Equal => 0,
 } * (1 << 23);

 let mut predicted_difference: i64 = 0;
 for i in 0..order {
  let srd = (reconstructed_differences[reconstructed_differences.len() - i - 1] >> 31) | 1;
  prediction.d_weight[i] = prediction.d_weight[i].wrapping_sub(rshift32(prediction.d_weight[i].wrapping_sub(srd.wrapping_mul(srd0)), 8));
  predicted_difference = predicted_difference.wrapping_add((reconstructed_differences[reconstructed_differences.len() - i - 1] as i64).wrapping_mul(prediction.d_weight[i] as i64));
 }

 prediction.predicted_difference = clip_intp2((predicted_difference >> 22) as i32, 23);
 prediction.predicted_sample = clip_intp2(predictor.wrapping_add(prediction.predicted_difference), 23);
}
