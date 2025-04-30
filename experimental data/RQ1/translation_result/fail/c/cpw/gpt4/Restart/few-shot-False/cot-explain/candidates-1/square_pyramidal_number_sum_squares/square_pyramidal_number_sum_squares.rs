
fn square_pyramidal_number_sum_squares(s: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut n: i32 = 1;

    while sum < s {
        sum = sum.wrapping_add(n.wrapping_mul(n));
        if sum == s {
            return n;
        }
        n = n.wrapping_add(1);
    }

    -1
}
