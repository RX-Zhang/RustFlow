
use std::u32;

fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2(n: u32) -> u32 {
    let mut res = 0;
    let mut x = 0;

    while (x * x) < n {
        let mut y = 0;
        while (x * x + y * y) < n {
            res += 1;
            y = y.wrapping_add(1);
        }
        x = x.wrapping_add(1);
    }

    res
}
