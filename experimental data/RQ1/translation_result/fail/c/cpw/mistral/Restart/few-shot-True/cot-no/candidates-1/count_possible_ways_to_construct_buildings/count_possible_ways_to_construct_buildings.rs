
use std::u32;

fn count_possible_ways_to_construct_buildings(n: u32) -> u32 {
    if n == 1 {
        return 4;
    }

    let mut count_b: u32 = 1;
    let mut count_s: u32 = 1;

    for _ in 2..=n {
        let prev_count_b = count_b;
        let prev_count_s = count_s;

        count_s = prev_count_b.wrapping_add(prev_count_s);
        count_b = prev_count_s;
    }

    (count_s.wrapping_add(count_b)).wrapping_mul(count_s.wrapping_add(count_b))
}
