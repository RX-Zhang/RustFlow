fn triangular_numbers_1(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    
    let c = num.wrapping_mul(-2);
    let b = 1;
    let a: i32 = 1;
    
    let d = b.wrapping_mul(b).wrapping_sub(4_i32.wrapping_mul(a).wrapping_mul(c));
    
    if d < 0 {
        return 0;
    }
    
    let root1 = (-b as f64 + (d as f64).sqrt()) / (2.0 * a as f64);
    let root2 = (-b as f64 - (d as f64).sqrt()) / (2.0 * a as f64);
    
    if !root1.is_finite() || !root2.is_finite() {
        return 0;
    }
    
    if root1 > 0.0 && root1.floor() == root1 {
        return 1;
    }
    
    if root2 > 0.0 && root2.floor() == root2 {
        return 1;
    }
    
    0
}