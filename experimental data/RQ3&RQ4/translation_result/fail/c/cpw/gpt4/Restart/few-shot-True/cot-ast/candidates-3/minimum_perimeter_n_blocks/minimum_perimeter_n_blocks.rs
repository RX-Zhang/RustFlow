
fn minimum_perimeter_n_blocks(n: i32) -> i32 {
    if n <= 0 {
        return 2;
    }
    let l = (n as f64).sqrt() as i32;
    let sq = l.wrapping_mul(l);
    if sq == n {
        return l.wrapping_mul(4);
    } else {
        let row = n / l;
        let mut perimeter = 2 * (l + row);
        if n % l != 0 {
            perimeter = perimeter.wrapping_add(2);
        }
        return perimeter;
    }
}
