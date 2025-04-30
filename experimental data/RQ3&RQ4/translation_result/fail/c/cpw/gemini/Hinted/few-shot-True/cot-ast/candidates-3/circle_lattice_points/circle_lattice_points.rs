
fn circle_lattice_points(r: i32) -> i32 {
    if r <= 0 {
        return 0;
    }
    let mut result = 4;
    for x in 1..r {
        let y_square = r * r - x * x;
        let y = (y_square as f64).sqrt() as i32;
        if y * y == y_square {
            result += 4;
        }
    }
    result
}
