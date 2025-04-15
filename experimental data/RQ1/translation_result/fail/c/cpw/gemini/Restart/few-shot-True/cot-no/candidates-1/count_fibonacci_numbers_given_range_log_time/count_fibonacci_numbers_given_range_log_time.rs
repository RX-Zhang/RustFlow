
fn count_fibonacci_numbers_given_range_log_time(low: i32, high: i32) -> i32 {
    let (mut f1, mut f2, mut f3) = (0, 1, 1);
    let mut result = 0;
    while f1 <= high {
        if f1 >= low {
            result += 1;
        }
        f1 = f2;
        f2 = f3;
        f3 = f1 + f2;
    }
    result
}
