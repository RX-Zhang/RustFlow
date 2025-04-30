fn triangular_numbers_1(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    let c: i32 = (-2_i32).wrapping_mul(num);
    let b: i32 = 1;
    let a: i32 = 1;
    let d: i32 = (b.wrapping_mul(b)).wrapping_sub((4_i32).wrapping_mul(a).wrapping_mul(c));
    if d < 0 {
        return 0;
    }
    let root1: f32 = ((-b as f32) + (d as f32).sqrt()) / (2 * a) as f32;
    let root2: f32 = ((-b as f32) - (d as f32).sqrt()) / (2 * a) as f32;
    if root1 > 0.0 && root1.floor() == root1 {
        return 1;
    }
    if root2 > 0.0 && root2.floor() == root2 {
        return 1;
    }
    0
}