
fn find_index_given_fibonacci_number_constant_time(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut res = 1;

    while c < n {
        c = a + b;
        res += 1;
        a = b;
        b = c;
    }

    res
}
