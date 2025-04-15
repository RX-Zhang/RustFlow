
use std::num::Wrapping;

fn primality_test_set_5using_lucas_lehmer_series(p: i32) -> i32 {
    if p <= 1 {
        return 0;
    }

    let check_number = Wrapping(2_i64.saturating_pow(p as u32).saturating_sub(1));
    let mut next_val = Wrapping(4_i64) % check_number;

    for _ in 1..p - 1 {
        next_val = (next_val * next_val - Wrapping(2)) % check_number;
    }

    if next_val.0 == 0 {
        1
    } else {
        0
    }
}
