fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx: i32 = 0;
    let mut i: i32 = nb_intervals.wrapping_shr(1);
    while i > 0 {
        let new_idx = idx.wrapping_add(i);
        let product = (factor as i64).wrapping_mul(intervals[new_idx as usize] as i64);
        let shifted = (value as i64).wrapping_shl(24);
        if product <= shifted {
            idx = new_idx;
        }
        i = i.wrapping_shr(1);
    }
    idx
}
