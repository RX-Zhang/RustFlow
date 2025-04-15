fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32]) -> i32 {
    let mut idx: usize = 0;
    let mut i = intervals.len() >> 1;

    while i > 0 {
        let product = (factor as i64).wrapping_mul(intervals[idx + i] as i64);
        let shifted = (value as i64).wrapping_shl(24);

        if product <= shifted {
            idx += i;
        }

        i >>= 1;
    }

    idx as i32
}
