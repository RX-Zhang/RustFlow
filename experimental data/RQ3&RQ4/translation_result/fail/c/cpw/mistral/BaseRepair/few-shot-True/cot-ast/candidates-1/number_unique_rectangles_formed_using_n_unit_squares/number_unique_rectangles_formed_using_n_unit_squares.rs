
use std::i32;
use std::ops::*;

fn number_unique_rectangles_formed_using_n_unit_squares(n: i32) -> i32 {
    let mut ans: i32 = 0; // specify the type of ans as i32
    let mut length = 1;
    while length <= (n as f64).sqrt() as i32 {
        let mut height = length;
        while (height as u32) <= (n as u32 / length as u32) {
            ans = ans.wrapping_add(1);
            height = height.wrapping_add(1);
        }
        length = length.wrapping_add(1);
    }
    ans
}
