
fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2_1(n: i32) -> i32 {
    let mut x: i32 = 0;
    let mut y_count: i32 = 0;
    let mut res: i32 = 0;

    while y_count.wrapping_mul(y_count) < n {
        y_count = y_count.wrapping_add(1);
    }

    while y_count != 0 {
        res = res.wrapping_add(y_count);
        x = x.wrapping_add(1);
        while y_count != 0 && (x.wrapping_mul(x).wrapping_add((y_count - 1).wrapping_mul(y_count - 1)) >= n) {
            y_count = y_count.wrapping_sub(1);
        }
    }

    res
}
