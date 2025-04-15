pub fn calculate_idx(quantized_sample: i32) -> usize {
    let idx = (quantized_sample ^ ((quantized_sample < 0) as i32 * -1)).wrapping_add(1) as usize;
    idx
}
