
fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2(n: i32) -> i32 {
    let mut res = 0;
    for x in 0.. {
        if x * x >= n {
            break;
        }
        for y in 0.. {
            if x * x + y * y >= n {
                break;
            }
            res += 1;
        }
    }
    res
}
