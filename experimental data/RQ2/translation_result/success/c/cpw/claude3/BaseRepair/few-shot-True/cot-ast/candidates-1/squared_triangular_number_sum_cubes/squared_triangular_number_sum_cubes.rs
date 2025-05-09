fn squared_triangular_number_sum_cubes(s: i32) -> i32 {
    let mut sum = 0;
    let mut n: i32 = 1;
    while sum < s {
        sum = sum.wrapping_add(n.wrapping_mul(n).wrapping_mul(n));
        if sum == s {
            return n;
        }
        n = n.wrapping_add(1);
    }
    -1
}