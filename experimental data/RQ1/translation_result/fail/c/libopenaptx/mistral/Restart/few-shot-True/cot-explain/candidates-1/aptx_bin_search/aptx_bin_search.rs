
use std::boxed::Box;

fn aptx_bin_search(value: i32, factor: i32, intervals: &Box<[i32]>, nb_intervals: usize) -> usize {
    let mut idx: usize = 0;
    let mut i = nb_intervals / 2;

    while i > 0 {
        if (factor as i64) * (intervals[idx + i] as i64) <= ((value as i64) << 24) {
            idx += i;
        }
        i >>= 1;
    }
    idx
}
