
fn triangular_numbers_1(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    let c = -2_i32.wrapping_mul(num);
    let b = 1_i32;
    let a = 1_i32;
    let disc = b.wrapping_mul(b).wrapping_sub(c.wrapping_mul(4));
    if disc < 0 {
        return 0;
    }
    let sqrt_disc = (disc as f64).sqrt();
    let root1 = (-(b as f64) + sqrt_disc) / (2.0 * a as f64);
    let root2 = (-(b as f64) - sqrt_disc) / (2.0 * a as f64);
    if root1 > 0.0 && (root1 - root1.floor()) < f64::EPSILON {
        return 1;
    }
    if root2 > 0.0 && (root2 - root2.floor()) < f64::EPSILON {
        return 1;
    }
    0
}
