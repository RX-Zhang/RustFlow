
fn nth_non_fibonacci_number(mut n: i32) -> i32 {
    let mut prev_prev = 1;
    let mut prev = 2;
    let mut curr = 3;
    while n > 0 {
        prev_prev = prev;
        prev = curr;
        curr = prev_prev + prev;
        n -= (curr - prev - 1);
    }
    n += (curr - prev - 1);
    prev + n
}
