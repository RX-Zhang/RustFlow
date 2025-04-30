

fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2_1(n: i32) -> i32 {
    let mut x: i32 = 0;
    let mut ycount: i32 = 0;
    let mut res: i32 = 0;

    while ycount * ycount < n {
        ycount += 1;
    }

    while ycount != 0 {
        res += ycount;
        x += 1;
        while ycount != 0 && (x * x + (ycount - 1) * (ycount - 1) >= n) {
            ycount -= 1;
        }
    }

    res
}
