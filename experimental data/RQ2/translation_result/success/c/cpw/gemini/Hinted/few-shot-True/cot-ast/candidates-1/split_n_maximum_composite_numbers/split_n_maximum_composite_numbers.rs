fn split_n_maximum_composite_numbers(n: i32) -> i32 {
    if n < 4 {
        return -1;
    }
    let rem: i32 = n.wrapping_rem(4);
    if rem == 0 {
        return n.wrapping_div(4);
    }
    if rem == 1 {
        if n < 9 {
            return -1;
        }
        return (n.wrapping_sub(9)).wrapping_div(4).wrapping_add(1);
    }
    if rem == 2 {
        return (n.wrapping_sub(6)).wrapping_div(4).wrapping_add(1);
    }
    if rem == 3 {
        if n < 15 {
            return -1;
        }
        return (n.wrapping_sub(15)).wrapping_div(4).wrapping_add(2);
    }
    -1
}