

fn count_possible_paths_top_left_bottom_right_nxm_matrix_3(m: i32, n: i32) -> i32 {
    let mut path: i32 = 1;
    for i in n..(m + n - 1) {
        path = path.wrapping_mul(i);
        path = path.wrapping_div(i - n + 1);
    }
    path
}
