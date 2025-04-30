
fn middle_of_three_using_minimum_comparisons_2(a: i32, b: i32, c: i32) -> i32 {
    let x = a.wrapping_sub(b);
    let y = b.wrapping_sub(c);
    let z = a.wrapping_sub(c);

    if x.wrapping_mul(y) > 0 {
        b
    } else if x.wrapping_mul(z) > 0 {
        c
    } else {
        a
    }
}
