
fn nth_non_fibonacci_number(n: i32) -> i32 {
    let mut prev_prev: i64 = 1;
    let mut prev: i64 = 2;
    let mut curr: i64 = 3;
    let mut n: i64 = n as i64;

    while n > 0 {
        prev_prev = prev;
        prev = curr;
        curr = prev_prev.wrapping_add(prev);
        n = n - (curr - prev - 1);
    }

    n = n + (curr - prev - 1);
    (prev + n as i64) as i32
}
