
fn split_n_maximum_composite_numbers(n: i32) -> i32 {
    if n < 4 {
        return -1;
    }
    let rem = n % 4;
    if rem == 0 {
        return n.wrapping_div(4);
    }
    if rem == 1 {
        if n < 9 {
            return -1;
        }
        return (n - 9).wrapping_div(4) + 1;
    }
    if rem == 2 {
        return (n - 6).wrapping_div(4) + 1;
    }
    if rem == 3 {
        if n < 15 {
            return -1;
        }
        return (n - 15).wrapping_div(4) + 2;
    }
    -1 // To satisfy the function's return type in case of unexpected input
}
