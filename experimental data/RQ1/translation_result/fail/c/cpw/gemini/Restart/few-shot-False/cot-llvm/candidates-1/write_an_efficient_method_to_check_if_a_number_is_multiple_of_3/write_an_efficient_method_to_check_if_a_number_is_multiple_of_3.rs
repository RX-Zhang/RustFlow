
fn write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(n: i32) -> bool {
    let mut odd_count = 0;
    let mut even_count = 0;
    let mut n = n;
    if n < 0 {
        return write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(-n);
    }
    if n == 0 {
        return true;
    }
    if n == 1 {
        return false;
    }
    while n > 0 {
        if n % 2 == 1 {
            odd_count += 1;
        }
        if n % 4 == 2 {
            even_count += 1;
        }
        n >>= 2;
    }
    let diff: i32 = odd_count - even_count;
    write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(diff.abs() as i32 % 3)
}
