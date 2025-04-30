
use std::num::Wrapping;

fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    if k <= 0 {
        return 0;
    }
    
    let mut p = Wrapping(1);
    
    for _ in 0..b {
        p *= Wrapping(a);
    }
    
    let mut count = 0;
    
    while p > Wrapping(0) && count < k {
        let rem = (p % Wrapping(10)).0;
        count += 1;
        
        if count == k {
            return rem;
        }
        
        p = p / Wrapping(10);
    }
    
    0
}
