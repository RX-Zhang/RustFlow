
use std::f64;

fn hardy_ramanujan_theorem(n: i32) -> i32 {
    let mut count = 0;
    let mut n = n;
    
    if n % 2 == 0 {
        count += 1;
        while n % 2 == 0 {
            n /= 2;
        }
    }
    
    let mut i = 3;
    while i <= (n as f64).sqrt() as i32 {
        if n % i == 0 {
            count += 1;
            while n % i == 0 {
                n /= i;
            }
        }
        i += 2;
    }
    
    if n > 2 {
        count += 1;
    }
    
    count
}
