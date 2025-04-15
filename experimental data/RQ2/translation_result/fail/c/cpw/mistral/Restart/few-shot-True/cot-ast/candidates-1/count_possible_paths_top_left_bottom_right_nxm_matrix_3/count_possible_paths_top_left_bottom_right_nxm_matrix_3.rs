
use std::num::Wrapping;

fn count_possible_paths_top_left_bottom_right_nxm_matrix_3(m: i32, n: i32) -> i32 {
    let mut path = Wrapping(1);
    for i in n..(m + n - 1) {
        path *= Wrapping(i);
        path /= Wrapping(i - n + 1);
    }
    path.0
}
