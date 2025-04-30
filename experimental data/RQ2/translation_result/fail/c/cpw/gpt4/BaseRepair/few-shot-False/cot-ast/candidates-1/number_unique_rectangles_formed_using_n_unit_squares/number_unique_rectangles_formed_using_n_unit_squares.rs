
use std::f64;

fn number_unique_rectangles_formed_using_n_unit_squares(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let limit = (n as f64).sqrt() as i32;
    for length in 1..=limit {
        for height in length.. {
            if height * length > n {
                break;
            }
            ans = ans.wrapping_add(1);
        }
    }
    ans
}
