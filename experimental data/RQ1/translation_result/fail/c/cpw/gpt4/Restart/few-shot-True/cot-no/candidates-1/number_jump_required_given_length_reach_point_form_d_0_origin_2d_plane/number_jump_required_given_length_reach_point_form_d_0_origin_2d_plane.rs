
use std::cmp::{min, max};

fn number_jump_required_given_length_reach_point_form_d_0_origin_2d_plane(a: i32, b: i32, d: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    let temp = a;
    a = min(a, b);
    b = max(temp, b);
    
    if d >= b {
        return (d + b - 1) / b;
    }
    if d == 0 {
        return 0;
    }
    if d == a {
        return 1;
    }
    2
}
