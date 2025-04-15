
use std::boxed::Box;

fn smallest_of_three_integers_without_comparison_operators(x: i32, y: i32, z: i32) -> i32 {
    let mut bx = Box::new(x);
    let mut by = Box::new(y);
    let mut bz = Box::new(z);
    let mut c: i32 = 0;

    while *bx != 0 && *by != 0 && *bz != 0 {
        *bx = bx.wrapping_sub(1);
        *by = by.wrapping_sub(1);
        *bz = bz.wrapping_sub(1);
        c = c.wrapping_add(1);
    }
    
    c
}
