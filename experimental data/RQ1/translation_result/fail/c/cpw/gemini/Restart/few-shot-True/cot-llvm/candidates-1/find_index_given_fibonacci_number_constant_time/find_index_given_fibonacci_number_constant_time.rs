

fn find_index_given_fibonacci_number_constant_time(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut a: i32 = 0;
    let mut b: i32 = 1;
    let mut c: i32 = 1;
    let mut res: i32 = 1;

    while c < n {
        c = a.wrapping_add(b);
        res = res.wrapping_add(1);
        a = b;
        b = c;
    }

    res
}
