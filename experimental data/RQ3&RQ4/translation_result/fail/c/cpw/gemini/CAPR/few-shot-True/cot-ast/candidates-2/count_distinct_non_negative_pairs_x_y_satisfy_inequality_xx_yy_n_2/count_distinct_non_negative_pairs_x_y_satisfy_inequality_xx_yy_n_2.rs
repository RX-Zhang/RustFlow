
fn count_distinct_non_negative_pairs_x_y_satisfy_inequality_xx_yy_n_2(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    
    let mut res: i32 = 0;
    let mut x: i32 = 0;
    
    while x * x < n {
        let mut y: i32 = 0;
        while x * x + y * y < n {
            res = res.wrapping_add(1);
            y = y.wrapping_add(1);
        }
        x = x.wrapping_add(1);
    }
    
    res
}
