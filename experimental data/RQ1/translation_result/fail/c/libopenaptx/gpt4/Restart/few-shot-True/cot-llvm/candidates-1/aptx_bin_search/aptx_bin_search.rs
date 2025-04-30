fn aptx_bin_search(value: i32, factor: i32, intervals: &Box<[i32]>, nb_intervals: i32) -> i32 {
    let mut idx: i32 = 0;
    let mut i: i32 = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64).wrapping_mul(intervals[idx as usize + i as usize] as i64) <= ((value as i64) << 24) {
            idx += i;
        }
        i >>= 1;
    }

    idx
}
