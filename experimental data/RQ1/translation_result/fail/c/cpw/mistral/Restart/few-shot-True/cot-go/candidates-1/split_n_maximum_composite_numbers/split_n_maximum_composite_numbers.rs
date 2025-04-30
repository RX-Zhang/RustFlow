
fn split_n_maximum_composite_numbers(n: i32) -> i32 {
    if n < 4 {
        return -1;
    }

    let rem = n % 4;

    match rem {
        0 => n / 4,
        1 => {
            if n < 9 {
                return -1;
            }
            (n.wrapping_sub(9) as u32).wrapping_div(4) as i32 + 1
        }
        2 => (n.wrapping_sub(6) as u32).wrapping_div(4) as i32 + 1,
        3 => {
            if n < 15 {
                return -1;
            }
            (n.wrapping_sub(15) as u32).wrapping_div(4) as i32 + 2
        }
        _ => unreachable!(),
    }
}
