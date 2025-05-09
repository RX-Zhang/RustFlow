
fn number_non_negative_integral_solutions_b_c_n(n: i32) -> i32 {
    let mut result: i32 = 0;
    for i in 0..=n {
        for j in 0..=(n - i) {
            for k in 0..=(n - i - j) {
                if i.wrapping_add(j).wrapping_add(k) == n {
                    result = result.wrapping_add(1)
                }
            }
        }
    }
    result
}
