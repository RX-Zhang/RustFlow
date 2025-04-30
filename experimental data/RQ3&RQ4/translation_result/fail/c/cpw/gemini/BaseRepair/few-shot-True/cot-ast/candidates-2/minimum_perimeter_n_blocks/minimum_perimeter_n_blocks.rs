
fn minimum_perimeter_n_blocks(n: i32) -> i32 {
    if n < 0 {
        return 2;
    }

    let l = (n as f64).sqrt() as i32;
    let sq = l * l;
    if sq == n {
        return l * 4;
    } else {
        let row = n / l;
        let perimeter = 2 * (l + row as i32);
        if n % l != 0 {
            return perimeter + 2;
        } else {
            return perimeter;
        }
    }
}
