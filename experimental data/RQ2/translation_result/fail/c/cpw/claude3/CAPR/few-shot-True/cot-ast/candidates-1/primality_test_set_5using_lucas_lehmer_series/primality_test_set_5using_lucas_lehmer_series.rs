
fn primality_test_set_5using_lucas_lehmer_series(p: i32) -> i32 {
    if p >= 63 {
        return 0;
    }
    
    let check_number = 2_i64.wrapping_pow(p as u32).wrapping_sub(1);
    let mut next_val = 4_i64.wrapping_rem(check_number);

    for _ in 1..p-1 {
        next_val = next_val.wrapping_mul(next_val).wrapping_sub(2).wrapping_rem(check_number);
    }

    if next_val == 0 {
        1
    } else {
        0
    }
}
