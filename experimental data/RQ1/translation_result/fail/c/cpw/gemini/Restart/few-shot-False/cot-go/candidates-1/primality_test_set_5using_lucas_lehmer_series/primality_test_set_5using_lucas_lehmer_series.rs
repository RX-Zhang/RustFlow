
use std::num::Wrapping;

fn primality_test_set_5using_lucas_lehmer_series(p: i32) -> i32 {
    let check_number = Wrapping(2_i64.pow(p as u32) - 1);
    let mut next_val = Wrapping(4) % check_number;

    for _ in 1..(p - 1) {
        next_val = (next_val * next_val - Wrapping(2)) % check_number;
    }

    if next_val.0 == 0 {
        1
    } else {
        0
    }
}
