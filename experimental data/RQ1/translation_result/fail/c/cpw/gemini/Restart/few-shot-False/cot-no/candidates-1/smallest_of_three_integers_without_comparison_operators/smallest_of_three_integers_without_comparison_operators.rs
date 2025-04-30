
fn smallest_of_three_integers_without_comparison_operators(x: i32, y: i32, z: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    let mut z = z;
    let mut c: i32 = 0; // Specify the type of c as i32
    while x != 0 && y != 0 && z != 0 {
        x = x.wrapping_sub(1);
        y = y.wrapping_sub(1);
        z = z.wrapping_sub(1);
        c = c.wrapping_add(1);
    }
    c
}
