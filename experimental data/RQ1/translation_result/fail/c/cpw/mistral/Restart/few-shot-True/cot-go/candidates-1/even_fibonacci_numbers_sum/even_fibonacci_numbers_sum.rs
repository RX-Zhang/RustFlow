

use std::i32;
use std::num::Wrapping;

fn even_fibonacci_numbers_sum(limit: i32) -> i32 {
    let mut current = Wrapping(0);
    let mut next = Wrapping(2);
    let mut sum: i32 = 0; // specify the type for sum
    
    while current.0 <= limit {
        if current.0 % 2 == 0 {
            sum = sum.wrapping_add(current.0);
        }
        
        let temp = current;
        current = next;
        next = Wrapping(next.0.wrapping_add(temp.0));
    }
    
    sum
}

