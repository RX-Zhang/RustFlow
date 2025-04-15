
fn primality_test_set_5using_lucas_lehmer_series(p: i32) -> i32 {
    let check_number = (1u128 << p) - 1;
    let mut nextval = 4u128 % check_number;
    
    for _ in 1..(p - 1) {
        nextval = (nextval.wrapping_mul(nextval).wrapping_sub(2)) % check_number;
    }
    
    if nextval == 0 { 1 } else { 0 }
}
