use std::cmp::min;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32]) -> usize {
    let nb_intervals = intervals.len();
    let mut idx: usize = 0;
    let mut i = nb_intervals / 2;

    for j in 0..min(32, nb_intervals.ilog2() as usize) { //Avoid unnecessary iterations
        if (factor as i64) * (intervals[idx + i] as i64) <= ((value as i64) << 24) {
            idx += i;
        }
        i >>= 1;
    }

    idx
}
