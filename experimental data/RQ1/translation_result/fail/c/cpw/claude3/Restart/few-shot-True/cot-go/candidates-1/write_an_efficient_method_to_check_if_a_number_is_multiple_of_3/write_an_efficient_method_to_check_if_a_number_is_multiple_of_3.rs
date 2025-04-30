
use std::num::Wrapping;

fn write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(n: i32) -> i32 {
    let mut odd_count = 0;
    let mut even_count = 0;
    let mut n = Wrapping(n.abs() as u32);
    
    if n.0 == 0 {
        return 1;
    }
    if n.0 == 1 {
        return 0;
    }
    
    while n.0 != 0 {
        if n.0 & 1 != 0 {
            odd_count += 1;
        }
        if n.0 & 2 != 0 {
            even_count += 1;
        }
        n = Wrapping(n.0.wrapping_shr(2));
    }
    
    let diff = if odd_count > even_count {
        odd_count - even_count
    } else {
        even_count - odd_count
    };
    
    write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(diff as i32)
}
