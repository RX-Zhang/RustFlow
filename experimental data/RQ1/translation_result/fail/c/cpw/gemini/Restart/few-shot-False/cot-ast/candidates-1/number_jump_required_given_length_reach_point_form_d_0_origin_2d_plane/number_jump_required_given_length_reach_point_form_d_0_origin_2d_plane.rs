
fn number_jump_required_given_length_reach_point_form_d_0_origin_2d_plane(a: i32, b: i32, d: i32) -> i32 {
    let temp = a;
    let a = a.min(b);
    let b = temp.max(b);
    if d >= b {
        (d + b - 1) / b
    } else if d == 0 {
        0
    } else if d == a {
        1
    } else {
        2
    }
}
