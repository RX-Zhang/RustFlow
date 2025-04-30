
fn squared_triangular_number_sum_cubes(s: i32) -> i32 {
    let mut sum: i32 = 0;
    for n in 1_i32.. {
        sum = sum.wrapping_add(n.wrapping_mul(n).wrapping_mul(n));
        if sum == s {
            return n;
        }
        if sum > s {
            break;
        }
    }
    -1
}
