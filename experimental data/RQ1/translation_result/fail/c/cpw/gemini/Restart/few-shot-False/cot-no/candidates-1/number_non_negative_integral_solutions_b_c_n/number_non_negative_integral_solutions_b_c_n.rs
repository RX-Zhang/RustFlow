
fn number_non_negative_integral_solutions_b_c_n(n: i32) -> i32 {
    let mut result = 0;
    for i in 0..=n {
        for j in 0..=(n - i) {
            for k in 0..=(n - i - j) {
                if i + j + k == n {
                    result += 1;
                }
            }
        }
    }
    result
}
