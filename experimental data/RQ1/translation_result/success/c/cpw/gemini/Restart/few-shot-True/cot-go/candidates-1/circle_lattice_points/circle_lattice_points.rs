
use std::num::Wrapping;

fn circle_lattice_points(r: i32) -> i32 {
    if r <= 0 {
        return 0;
    }

    let mut result = Wrapping(4);
    for x in 1..r {
        let y_square = Wrapping(r * r) - Wrapping(x * x);
        let y = (y_square.0 as f64).sqrt() as i32;
        if y * y == y_square.0 {
            result += Wrapping(4);
        }
    }

    result.0
}
