
fn count_possible_paths_top_left_bottom_right_nxm_matrix_3(m: i32, n: i32) -> i32 {
    if m < n {
        return 0;
    }

    let mut path = 1;
    for i in n..m + n - 1 {
        if i > i32::MAX / path {
            return 0;
        }
        path = path.wrapping_mul(i);
        if (i - n + 1) > 1 {
            path = path.wrapping_div(i - n + 1);
        }
    }
    path
}
