
use std::cmp::min;
use std::cmp::max;
use std::num::Wrapping;

fn number_jump_required_given_length_reach_point_form_d_origin_2d_plane(
    a: i32,
    b: i32,
    d: i32,
) -> i32 {
    let mut a_mut = a;
    let mut b_mut = b;

    a_mut = min(a_mut, b_mut);
    b_mut = max(a_mut, b_mut);

    if d >= b_mut {
        return (d + b_mut - 1) / b_mut;
    }
    if d == 0 {
        return 0;
    }

    let mut result = 0;
    let mut d_mut = Wrapping(d);
    while d_mut >= Wrapping(b_mut) {
        result += 1;
        d_mut -= Wrapping(b_mut - a_mut);
    }

    result + d_mut.0 as i32 / b_mut
}
