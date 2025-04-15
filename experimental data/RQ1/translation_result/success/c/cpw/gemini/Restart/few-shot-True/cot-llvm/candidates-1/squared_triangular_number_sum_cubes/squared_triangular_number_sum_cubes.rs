
fn squared_triangular_number_sum_cubes(s: i32) -> i32 {
    let mut sum = 0;
    let mut n = 1;
    while sum < s {
        sum = sum.wrapping_add(n * n * n);
        if sum == s {
            return n;
        }
        n += 1;
    }
    -1
}
