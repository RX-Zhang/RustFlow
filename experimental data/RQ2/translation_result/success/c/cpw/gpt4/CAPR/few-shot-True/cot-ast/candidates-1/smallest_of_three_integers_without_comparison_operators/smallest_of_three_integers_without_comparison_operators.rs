
use std::num::Wrapping;

fn smallest_of_three_integers_without_comparison_operators(x: i32, y: i32, z: i32) -> i32 {
    let mut x = Wrapping(x);
    let mut y = Wrapping(y);
    let mut z = Wrapping(z);
    let mut c = Wrapping(0);

    while x.0 != 0 && y.0 != 0 && z.0 != 0 {
        x -= Wrapping(1);
        y -= Wrapping(1);
        z -= Wrapping(1);
        c += Wrapping(1);
    }

    c.0
}
