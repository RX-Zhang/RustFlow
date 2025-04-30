
use std::num::Wrapping;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        if Wrapping(factor as i64) * Wrapping(intervals[idx as usize + i as usize] as i64) <= Wrapping((value as i64) << 24) {
            idx += i;
        }
        i >>= 1;
    }

    idx
}
