fn modular_exponentiation_power_in_modular_arithmetic(x: i32, y: u32, p: i32) -> i32 {
    let mut res: i32 = 1;
    let mut x_mut = x.wrapping_rem(p);
    let mut y_mut = y;

    while y_mut > 0 {
        if y_mut & 1 != 0 {
            res = res.wrapping_mul(x_mut).wrapping_rem(p);
        }
        y_mut = y_mut.wrapping_shr(1);
        x_mut = x_mut.wrapping_mul(x_mut).wrapping_rem(p);
    }

    res
}