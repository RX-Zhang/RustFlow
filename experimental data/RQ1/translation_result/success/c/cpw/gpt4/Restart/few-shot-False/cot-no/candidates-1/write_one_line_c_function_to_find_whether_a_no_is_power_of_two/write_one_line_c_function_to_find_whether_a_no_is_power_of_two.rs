
fn write_one_line_c_function_to_find_whether_a_no_is_power_of_two(mut n: i32) -> bool {
    if n == 0 {
        return false;
    }
    while n != 1 {
        if n % 2 != 0 {
            return false;
        }
        n = n / 2;
    }
    true
}
