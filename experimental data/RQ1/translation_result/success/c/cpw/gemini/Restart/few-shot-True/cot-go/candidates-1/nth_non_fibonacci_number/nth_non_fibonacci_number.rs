

fn nth_non_fibonacci_number(n: i32) -> i32 {
    let mut prev_prev: i64 = 1;
    let mut prev: i64 = 2;
    let mut curr: i64 = 3;
    let mut n = n as i64;

    while n > 0 {
        let temp = prev;
        prev = curr;
        curr = temp + prev;
        n -= (curr - prev - 1) as i64;
    }

    n += (curr - prev - 1) as i64;
    (prev + n) as i32
}