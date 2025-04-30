

fn write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(n: i32) -> i32 {
    let mut odd_count = 0;
    let mut even_count = 0;
    let n = if n < 0 { -n } else { n };
    
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 0;
    }

    let mut n = n;

    while n != 0 {
        if n & 1 != 0 {
            odd_count += 1;
        }
        if n & 2 != 0 {
            even_count += 1;
        }
        n = n.wrapping_shr(2);
    }

    write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(odd_count - even_count)
}
