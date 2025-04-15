
use std::cmp::min;
use std::i32;

fn minimum_perimeter_n_blocks(n: i32) -> i32 {
    let l = (n as f64).sqrt() as i32;
    let sq = l * l;
    if sq == n {
        return l * 4;
    }

    let row = n / l;
    let perimeter = 2 * (l + row);
    let remainder = n % l;
    if remainder != 0 {
        return perimeter + 2;
    }

    min(perimeter, {
        let shifted = ((l + (row - 1)) << 1) as i32;
        shifted + 2
    })
}
