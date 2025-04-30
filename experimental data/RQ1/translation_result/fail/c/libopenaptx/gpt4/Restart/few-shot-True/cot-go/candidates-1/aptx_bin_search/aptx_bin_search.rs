fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> i32 {
    let mut idx: i32 = 0;
    let mut i: usize;

    i = nb_intervals >> 1;
    while i > 0 {
        if (factor as i64).wrapping_mul(intervals[idx as usize + i] as i64) <= (value as i64).wrapping_shl(24) {
            idx += i as i32;
        }
        i >>= 1;
    }

    idx
}
