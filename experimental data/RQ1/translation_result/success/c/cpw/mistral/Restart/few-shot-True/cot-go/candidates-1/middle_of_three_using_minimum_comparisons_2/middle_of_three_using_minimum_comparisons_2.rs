
fn middle_of_three_using_minimum_comparisons_2(a: i32, b: i32, c: i32) -> i32 {
    let x = i32::wrapping_sub(a, b);
    let y = i32::wrapping_sub(b, c);
    let z = i32::wrapping_sub(a, c);

    if x.wrapping_mul(y) > 0 {
        b
    } else if x.wrapping_mul(z) > 0 {
        c
    } else {
        a
    }
}
