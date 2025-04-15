
use std::i32;

fn count_possible_ways_to_construct_buildings(n: i32) -> i32 {
    if n == 1 {
        return 4;
    }

    let mut count_b: i32 = 1;
    let mut count_s: i32 = 1;
    let mut prev_count_b: i32 = 0;
    let mut prev_count_s: i32 = 0;

    for i in 2..=n {
        prev_count_b = count_b;
        prev_count_s = count_s;
        count_s = prev_count_b.wrapping_add(prev_count_s);
        count_b = prev_count_s;
    }

    let result = count_s.wrapping_add(count_b);
    result.wrapping_mul(result)
}
