
use std::f64;

fn program_check_plus_perfect_number(x: i32) -> bool {
    let mut temp = x;
    let mut n: i32 = 0;
    while temp != 0 {
        temp /= 10;
        n = n.wrapping_add(1);
    }
    
    temp = x;
    let mut sum: i32 = 0;
    while temp != 0 {
        sum = sum.wrapping_add((f64::powi((temp % 10).into(), n)).round() as i32);
        temp /= 10;
    }
    
    sum == x
}
