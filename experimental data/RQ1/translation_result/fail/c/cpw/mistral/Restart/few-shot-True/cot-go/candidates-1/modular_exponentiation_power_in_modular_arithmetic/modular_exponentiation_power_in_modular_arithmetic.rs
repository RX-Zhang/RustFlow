
use std::num::Wrapping;

fn modular_exponentiation_power_in_modular_arithmetic(x: i32, y: u32, p: i32) -> i32 {
    if p == 0 {
        return 0; // Handle division by zero
    }

    let mut res = Wrapping(1);
    let mut x = Wrapping(x % p);
    let mut y = y;

    while y > 0 {
        if y & 1 != 0 {
            res = Wrapping((res.0 as i64 * x.0 as i64 % p as i64) as i32);
        }
        y = y >> 1;
        x = Wrapping((x.0 as i64 * x.0 as i64 % p as i64) as i32);
    }

    res.0
}
