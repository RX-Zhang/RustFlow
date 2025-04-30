
fn write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(n: i32) -> i32 {
    let mut odd_count: i32 = 0;
    let mut even_count: i32 = 0;
    let mut n = n.abs();
    
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 0;
    }

    while n != 0 {
        if n & 1 != 0 {
            odd_count = odd_count.wrapping_add(1);
        }
        if n & 2 != 0 {
            even_count = even_count.wrapping_add(1);
        }
        n = (n as u32).wrapping_shr(2) as i32;
    }

    let diff = (odd_count - even_count).abs();
    write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(diff)
}
