
fn square_pyramidal_number_sum_squares(s: i64) -> i64 {
    let mut sum: i64 = 0;
    let mut n: i64 = 1;
    loop {
        sum = sum.wrapping_add(n * n);
        if sum == s {
            return n;
        } else if sum > s {
            return -1;
        }
        n += 1;
    }
}
