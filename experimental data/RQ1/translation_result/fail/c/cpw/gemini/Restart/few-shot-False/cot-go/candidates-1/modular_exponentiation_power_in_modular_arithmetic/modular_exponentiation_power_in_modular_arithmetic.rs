
pub fn modular_exponentiation_power_in_modular_arithmetic(x: i32, y: u32, p: i32) -> i32 {
    let mut res = 1;
    let mut x = x % p;
    let mut y = y;
    while y > 0 {
        if y & 1 == 1 {
            res = (res as i64 * x as i64) as i32 % p;
        }
        y >>= 1;
        x = (x as i64 * x as i64) as i32 % p;
    }
    res
}
