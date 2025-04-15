
fn split_n_maximum_composite_numbers(n: i32) -> i32 {
    if n < 4 {
        return -1;
    }
    let rem = n % 4;
    match rem {
        0 => n / 4,
        1 => {
            if n < 9 {
                -1
            } else {
                (n - 9) / 4 + 1
            }
        }
        2 => (n - 6) / 4 + 1,
        3 => {
            if n < 15 {
                -1
            } else {
                (n - 15) / 4 + 2
            }
        }
        _ => unreachable!(),
    }
}
