
use std::cmp;

fn number_jump_required_given_length_reach_point_form_d_0_origin_2d_plane(a: i32, b: i32, d: i32) -> i32 {
    let min_val = cmp::min(a, b);
    let max_val = cmp::max(a, b);
    if d >= max_val {
        (d + max_val - 1) / max_val
    } else if d == 0 {
        0
    } else if d == min_val {
        1
    } else {
        2
    }
}
