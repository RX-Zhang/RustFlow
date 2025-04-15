
fn primality_test_set_1_introduction_and_school_method_1(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }
    if n <= 3 {
        return 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return 0;
    }
    let mut i: i64 = 5;
    while i.checked_mul(i).map_or(false, |i_squared| i_squared <= n as i64) {
        if n % i as i32 == 0 || n % (i as i32 + 2) == 0 {
            return 0;
        }
        i = i.wrapping_add(6);
    }
    1
}
