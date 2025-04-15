
use std::boxed::Box;

fn sum_factors_number(n: i32) -> i32 {
    let mut result: i32 = 0;
    let square_root_n = (n as f64).sqrt() as i32;
    for i in 2..=square_root_n {
        if n % i == 0 {
            if i == n / i {
                result = result.wrapping_add(i);
            } else {
                result = result.wrapping_add(i).wrapping_add(n / i);
            }
        }
    }
    let result_ptr = Box::new(result.wrapping_add(n).wrapping_add(1));
    *result_ptr
}
