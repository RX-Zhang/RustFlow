
fn smallest_of_three_integers_without_comparison_operators(mut x: i32, mut y: i32, mut z: i32) -> i32 {
    let mut c: i32 = 0;
    while x != 0 && y != 0 && z != 0 {
        x = x.wrapping_sub(1);
        y = y.wrapping_sub(1);
        z = z.wrapping_sub(1);
        c = c.wrapping_add(1);
    }
    c
}
