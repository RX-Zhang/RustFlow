
use std::num::Wrapping;

fn smallest_of_three_integers_without_comparison_operators(x: i32, y: i32, z: i32) -> i32 {
    let mut x_wrapped = Wrapping(x);
    let mut y_wrapped = Wrapping(y);
    let mut z_wrapped = Wrapping(z);

    let mut count = 0;

    while x_wrapped.0 != 0 && y_wrapped.0 != 0 && z_wrapped.0 != 0 {
        x_wrapped = Wrapping(x_wrapped.0.wrapping_sub(1));
        y_wrapped = Wrapping(y_wrapped.0.wrapping_sub(1));
        z_wrapped = Wrapping(z_wrapped.0.wrapping_sub(1));
        count += 1;
    }

    count
}
