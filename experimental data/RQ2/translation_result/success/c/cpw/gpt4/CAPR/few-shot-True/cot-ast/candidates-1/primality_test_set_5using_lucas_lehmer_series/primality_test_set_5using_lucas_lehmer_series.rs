

fn primality_test_set_5using_lucas_lehmer_series(p: i32) -> i32 {
    let check_number = (2u64.pow(p as u32) - 1) as i64;
    let mut nextval = 4 % check_number;

    for _ in 1..(p - 1) {
        nextval = (nextval.wrapping_mul(nextval) - 2) % check_number;
    }

    if nextval == 0 {
        return 1;
    } else {
        return 0;
    }
}
