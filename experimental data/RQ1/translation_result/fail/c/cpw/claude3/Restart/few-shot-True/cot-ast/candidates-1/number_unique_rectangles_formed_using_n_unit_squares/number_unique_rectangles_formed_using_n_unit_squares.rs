
fn number_unique_rectangles_formed_using_n_unit_squares(n: i32) -> i32 {
    let mut ans: i32 = 0;
    for length in 1..=(n as f64).sqrt() as i32 {
        for height in length..=n {
            if height * length <= n {
                ans = ans.wrapping_add(1);
            } else {
                break;
            }
        }
    }
    ans
}
