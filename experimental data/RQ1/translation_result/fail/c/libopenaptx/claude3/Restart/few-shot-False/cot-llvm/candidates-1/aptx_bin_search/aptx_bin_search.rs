use std::cmp::Ordering;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx: i32 = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        let product = (factor as i64).wrapping_mul(intervals[idx.wrapping_add(i) as usize] as i64);
        let shifted_value = (value as i64).wrapping_shl(24);

        if product <= shifted_value {
            idx = idx.wrapping_add(i);
        }

        i = i.wrapping_shr(1);
    }

    idx
}
