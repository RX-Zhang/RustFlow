
use std::f64;

fn sum_factors_number(n: i32) -> i32 {
    let mut result: i32 = 0; // Specify the type for the binding
    let sqrt_n = (n as f64).sqrt() as i32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            if i == n / i {
                result = result.wrapping_add(i);
            } else {
                result = result.wrapping_add(i).wrapping_add(n / i);
            }
        }
    }
    result.wrapping_add(n).wrapping_add(1)
}
