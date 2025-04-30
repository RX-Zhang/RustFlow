
use std::ops::*;
use std::cmp::*;
use std::fmt::*;

fn triangular_numbers1(num: i32) -> i32 {
    if num < 0 {
        0
    } else {
        let c = -2 * num;
        let b = 1;
        let a = 1;
        let d = i32::wrapping_mul(b, b) - i32::wrapping_mul(4 * a, c);
        let root1 = (b as f32).powf(2.0) - d as f32;
        let root2 = root1.abs();
        if root1.is_sign_positive() {
            1
        } else if root2.is_sign_positive() {
            1
        } else {
            0
        }
    }
}
