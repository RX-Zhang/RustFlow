
fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2(n: i32) -> i32 {
    let mut res: i32 = 0;
    let mut x = 0;
    while x * x < n {
        let mut y = 0;
        while x * x + y * y < n {
            res = res.wrapping_add(1);
            y += 1;
        }
        x += 1;
    }
    res
}
