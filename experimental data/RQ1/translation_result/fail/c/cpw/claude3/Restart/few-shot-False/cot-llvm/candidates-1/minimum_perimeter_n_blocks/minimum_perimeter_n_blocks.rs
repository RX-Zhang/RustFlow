
use std::cmp;

fn minimum_perimeter_n_blocks(n: i32) -> i32 {
    let l = (n as f64).sqrt() as i32;
    let sq = l.wrapping_mul(l);
    
    if sq == n {
        l.wrapping_mul(4)
    } else {
        let row = n.wrapping_div(l);
        let mut perimeter = 2_i32.wrapping_mul(l.wrapping_add(row));
        
        if n.wrapping_rem(l) != 0 {
            perimeter = perimeter.wrapping_add(2);
        }
        
        cmp::max(perimeter, 2) // Ensure the result is at least 2
    }
}
