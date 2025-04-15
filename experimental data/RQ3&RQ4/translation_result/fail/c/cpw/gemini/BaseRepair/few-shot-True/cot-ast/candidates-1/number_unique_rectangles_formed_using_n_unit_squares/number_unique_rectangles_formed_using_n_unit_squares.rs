

fn number_unique_rectangles_formed_using_n_unit_squares(n: i32) -> i32 {
    let mut ans: i32 = 0;
    for length in 1..=(n as f64).sqrt() as i32 {
        let mut height: i32 = length;
        while height.wrapping_mul(length) <= n {
            ans = ans.wrapping_add(1);
            height = height.wrapping_add(1);
        }
    }
    ans
}