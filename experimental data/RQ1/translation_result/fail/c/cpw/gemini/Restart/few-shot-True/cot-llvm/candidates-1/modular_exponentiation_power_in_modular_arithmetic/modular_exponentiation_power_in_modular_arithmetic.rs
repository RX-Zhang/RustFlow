
use std::num::Wrapping;

fn modular_exponentiation_power_in_modular_arithmetic(x: i32, y: u32, p: i32) -> i32 {
    let mut res: Wrapping<i32> = Wrapping(1);
    let mut x_: Wrapping<i32> = Wrapping(x % p);
    let mut y_ = y;

    while y_ > 0 {
        if y_ % 2 == 1 {
            res = (res * x_) % Wrapping(p);
        }
        y_ >>= 1;
        x_ = (x_ * x_) % Wrapping(p);
    }

    res.0
}
