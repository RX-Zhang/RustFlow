
use std::cmp::min;

fn minimum_perimeter_n_blocks(n: i32) -> i32 {
    let l = (n as f64).sqrt() as i32;
    let sq = l * l;
    if sq == n {
        l * 4
    } else {
        let row = n / l;
        let mut perimeter = 2 * (l + row);
        if n % l != 0 {
            perimeter += 2;
        }
        min(perimeter, i32::MAX)
    }
}
