
use std::f64;

fn number_unique_rectangles_formed_using_n_unit_squares(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let sqrt_n = f64::from(n).sqrt() as i32;
    
    for length in 1..=sqrt_n {
        for height in length..=n {
            if height.wrapping_mul(length) <= n {
                ans = ans.wrapping_add(1);
            } else {
                break;
            }
        }
    }
    
    ans
}
