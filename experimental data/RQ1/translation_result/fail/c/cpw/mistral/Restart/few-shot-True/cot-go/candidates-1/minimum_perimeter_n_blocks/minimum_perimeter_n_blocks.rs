
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
    let rem = n % l;
    let perimeter_adj = if rem != 0 { perimeter + 2 } else { perimeter };

    min(perimeter_adj, (l * 4).wrapping_shr(1) + (row * 4).wrapping_shr(1) + 4)
}
