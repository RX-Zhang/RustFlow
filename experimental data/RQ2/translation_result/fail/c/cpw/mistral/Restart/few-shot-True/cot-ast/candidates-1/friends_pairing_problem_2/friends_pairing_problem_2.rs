
use std::i32;

fn friends_pairing_problem2(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 2;
    let mut c = 0;

    if n <= 2 {
        return n;
    }

    for i in 3..=n {
        c = i32::wrapping_add(b, (i - 1) * a);
        a = b;
        b = c;
    }

    c
}
