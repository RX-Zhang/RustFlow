
fn square_pyramidal_number_sum_squares(s: i32) -> i32 {
    let mut sum = 0;
    for n in 1.. {
        sum += n * n;
        if sum == s {
            return n;
        }
    }
    -1
}
