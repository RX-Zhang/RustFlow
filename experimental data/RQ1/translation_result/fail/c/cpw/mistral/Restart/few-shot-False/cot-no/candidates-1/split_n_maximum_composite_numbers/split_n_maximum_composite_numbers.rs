
use std::ops::Shr;

fn split_n_maximum_composite_numbers(n: i32) -> i32 {
    if n < 4 {
        return -1;
    }
    let rem = n % 4;
    if rem == 0 {
        return n / 4;
    } else if rem == 1 {
        if n < 9 {
            return -1;
        }
        return (n.wrapping_sub(9)).shr(2) + 1;
    } else if rem == 2 {
        return (n.wrapping_sub(6)).shr(2) + 1;
    } else if rem == 3 {
        if n < 15 {
            return -1;
        }
        return (n.wrapping_sub(15)).shr(2) + 2;
    } else {
        panic!("Unexpected remainder");
    }
}
