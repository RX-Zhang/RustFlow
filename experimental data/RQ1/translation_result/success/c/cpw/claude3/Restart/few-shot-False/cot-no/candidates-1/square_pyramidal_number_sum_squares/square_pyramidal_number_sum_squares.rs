
fn square_pyramidal_number_sum_squares(s: i32) -> i32 {
    let mut sum: i32 = 0;
    for n in 1..=i32::MAX {
        sum = sum.wrapping_add(n.wrapping_mul(n));
        if sum == s {
            return n;
        }
        if sum > s {
            break;
        }
    }
    -1
}
