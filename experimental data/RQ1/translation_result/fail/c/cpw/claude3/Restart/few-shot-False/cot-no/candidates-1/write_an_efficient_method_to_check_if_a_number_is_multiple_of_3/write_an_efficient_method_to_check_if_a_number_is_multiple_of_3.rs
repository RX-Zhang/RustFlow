
use std::num::Wrapping;

fn write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(mut n: i32) -> i32 {
    let mut odd_count: i32 = 0;
    let mut even_count: i32 = 0;
    if n < 0 {
        n = n.wrapping_neg();
    }
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
        n = (Wrapping(n) >> 2).0;
    }
    write_an_efficient_method_to_check_if_a_number_is_multiple_of_3(odd_count.wrapping_sub(even_count).abs())
}
