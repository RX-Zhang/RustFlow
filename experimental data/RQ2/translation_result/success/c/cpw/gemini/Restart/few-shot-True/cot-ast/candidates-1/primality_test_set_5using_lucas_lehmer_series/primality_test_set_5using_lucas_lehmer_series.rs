
fn primality_test_set_5using_lucas_lehmer_series(p: i32) -> bool {
    if p <= 0 || p % 2 == 0 {
        return false; // Lucas-Lehmer test only applies to positive odd primes
    }

    let check_number = (2_i64.pow(p as u32) - 1);
    let mut nextval = 4 % check_number;
    for _ in 1..p - 1 {
        nextval = (nextval * nextval - 2) % check_number;
    }
    nextval == 0
}
