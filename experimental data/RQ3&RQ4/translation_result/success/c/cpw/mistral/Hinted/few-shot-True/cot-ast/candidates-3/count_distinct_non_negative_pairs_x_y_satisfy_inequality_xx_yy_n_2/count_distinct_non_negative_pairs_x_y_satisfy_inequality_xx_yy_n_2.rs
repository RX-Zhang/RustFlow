
use std::u32;

fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2(n: u32) -> u32 {
    let mut res: u32 = 0; // specify the type for res as u32
    let n_sq = n * n;
    for x in 0.. {
        let x_sq = x * x;
        if x_sq > n_sq {
            break;
        }
        for y in 0.. {
            let y_sq = y * y;
            if x_sq + y_sq > n_sq {
                break;
            }
            res = res.wrapping_add(1);
        }
    }
    res
}
