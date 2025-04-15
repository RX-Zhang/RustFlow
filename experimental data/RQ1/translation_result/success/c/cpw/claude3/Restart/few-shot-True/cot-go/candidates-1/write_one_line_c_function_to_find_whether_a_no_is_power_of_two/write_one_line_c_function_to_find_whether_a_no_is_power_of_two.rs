
fn write_one_line_c_function_to_find_whether_a_no_is_power_of_two(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut num = n;
    while num != 1 {
        if num % 2 != 0 {
            return 0;
        }
        num = num.wrapping_div(2);
    }
    1
}
