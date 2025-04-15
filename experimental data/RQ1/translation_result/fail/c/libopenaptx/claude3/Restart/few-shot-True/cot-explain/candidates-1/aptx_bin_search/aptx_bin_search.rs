
use std::cmp::Ordering;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx: i32 = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        let left = (factor as i64).wrapping_mul(intervals[idx.wrapping_add(i) as usize] as i64);
        let right = (value as i64).wrapping_shl(24);
        
        if left <= right {
            idx = idx.wrapping_add(i);
        }
        
        i >>= 1;
    }

    idx
}
