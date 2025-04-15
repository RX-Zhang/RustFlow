
use std::num::Wrapping;

fn friends_pairing_problem_2(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut a: Wrapping<i32> = Wrapping(1);
    let mut b: Wrapping<i32> = Wrapping(2);
    let mut c: Wrapping<i32> = Wrapping(0);

    for i in 3..=n {
        c = b + (Wrapping(i) - Wrapping(1)) * a;
        a = b;
        b = c;
    }

    c.0
}
