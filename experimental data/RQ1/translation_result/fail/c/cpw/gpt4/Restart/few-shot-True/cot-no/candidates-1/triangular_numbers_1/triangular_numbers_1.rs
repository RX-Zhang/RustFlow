
use std::f32;

fn triangular_numbers_1(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    let c = (-2 * num) as f32;
    let b = 1.0;
    let a = 1.0;
    let d = (b * b) - (4.0 * a * c);
    if d < 0.0 {
        return 0;
    }
    let root1 = (-b + d.sqrt()) / (2.0 * a);
    let root2 = (-b - d.sqrt()) / (2.0 * a);
    if root1 > 0.0 && root1.floor() == root1 {
        return 1;
    }
    if root2 > 0.0 && root2.floor() == root2 {
        return 1;
    }
    0
}
