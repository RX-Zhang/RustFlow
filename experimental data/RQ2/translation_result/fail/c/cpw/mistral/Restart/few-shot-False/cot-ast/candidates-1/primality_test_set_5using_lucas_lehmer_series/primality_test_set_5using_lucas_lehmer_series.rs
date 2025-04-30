
use std::i128;
use std::ops::Rem;

fn primality_test_set_5_using_lucas_lehmer_series(p: i32) -> bool {
    let check_number: i128 = 1i128 << p - 1;
    let mut next_val: i128 = 4 % check_number;

    for _ in 1..p - 1 {
        next_val = (next_val.rem(check_number) * next_val.rem(check_number) - 2i128) % check_number;
    }

    next_val == 0
}
