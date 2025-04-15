
fn primality_test_set_5using_lucas_lehmer_series(p: i32) -> i32 {
    let check_number: i64 = (2i64.pow(p as u32)) - 1;
    let mut nextval: i64 = 4 % check_number;

    for i in 1..p - 1 {
        nextval = (nextval.wrapping_mul(nextval) - 2) % check_number;
    }

    if nextval == 0 {
        1
    } else {
        0
    }
}
