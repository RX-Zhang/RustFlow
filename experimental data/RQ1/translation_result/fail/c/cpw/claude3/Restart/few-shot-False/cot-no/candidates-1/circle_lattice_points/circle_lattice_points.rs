
fn circle_lattice_points(r: i32) -> i32 {
    if r <= 0 {
        return 0;
    }

    let mut result: i32 = 4;
    let mut x = 1;

    while x < r {
        let y_square = r.wrapping_mul(r).wrapping_sub(x.wrapping_mul(x));
        let y = (y_square as f64).sqrt() as i32;
        
        if y.wrapping_mul(y) == y_square {
            result = result.wrapping_add(4);
        }
        
        x = x.wrapping_add(1);
    }

    result
}
