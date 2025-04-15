
fn write_one_line_c_function_to_find_whether_a_no_is_power_of_two(n: i32) -> bool {
    (n != 0) && ((n.wrapping_shr(31) % 2) as i32 == 0) && ((n & (n - 1)) == 0)
}
