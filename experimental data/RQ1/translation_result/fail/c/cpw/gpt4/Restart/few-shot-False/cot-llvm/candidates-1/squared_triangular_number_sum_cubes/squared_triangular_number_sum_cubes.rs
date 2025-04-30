
use std::num::Wrapping;

fn squared_triangular_number_sum_cubes(s: i32) -> i32 {
    let mut sum = Wrapping(0);
    let mut n = Wrapping(1);
    
    while sum < Wrapping(s) {
        sum = sum + (n * n * n);
        if sum.0 == s {
            return n.0;
        }
        n = n + Wrapping(1);
    }
    
    -1
}
