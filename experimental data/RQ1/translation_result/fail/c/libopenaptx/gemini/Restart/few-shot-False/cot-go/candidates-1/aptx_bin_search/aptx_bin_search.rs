fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx: i32 = 0;
    let mut i: i32 = nb_intervals >> 1;

    while i > 0 {
        if (i64::from(factor).wrapping_mul(i64::from(intervals[(idx + i) as usize]))) <= (i64::from(value).wrapping_shl(24 % 64)) {
            idx = idx.wrapping_add(i);
        }
        i >>= 1;
    }

    idx
}
