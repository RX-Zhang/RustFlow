
use std::boxed::Box;

fn number_unique_rectangles_formed_using_n_unit_squares(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let n_sqrt = (n as f64).sqrt().floor() as i32;
    for length in 1..=n_sqrt {
        for height in length.. {
            if height * length > n {
                break;
            }
            ans = ans.wrapping_add(1);
        }
    }
    ans
}
