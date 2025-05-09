fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2_1(n: i32) -> i32 {
    let mut x: i32 = 0;
    let mut y_count: i32 = 0;
    let mut res: i32 = 0;

    while y_count * y_count < n {
        y_count += 1;
    }

    let mut y_count = y_count;
    while y_count != 0 {
        res = res.wrapping_add(y_count);
        x += 1;
        while y_count != 0 && (x.wrapping_mul(x).wrapping_add((y_count - 1).wrapping_mul(y_count - 1)) >= n) {
            y_count -= 1;
        }
    }
    res
}