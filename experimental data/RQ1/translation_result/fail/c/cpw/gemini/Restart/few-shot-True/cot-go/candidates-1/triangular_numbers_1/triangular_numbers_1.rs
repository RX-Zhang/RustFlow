
fn triangular_numbers_1(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    let c = -2 * num;
    let b = 1;
    let a = 1;
    let d = b * b - 4 * a * c;
    if d < 0 {
        return 0;
    }
    let root1 = (-b as f64 + (d as f64).sqrt()) / (2.0 * a as f64);
    let root2 = (-b as f64 - (d as f64).sqrt()) / (2.0 * a as f64);
    if root1 > 0.0 && root1.fract() == 0.0 {
        return 1;
    }
    if root2 > 0.0 && root2.fract() == 0.0 {
        return 1;
    }
    0
}
