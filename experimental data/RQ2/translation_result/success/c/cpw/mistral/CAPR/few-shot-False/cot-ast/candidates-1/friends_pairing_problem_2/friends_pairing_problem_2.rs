
fn friends_pairing_problem_2(n: i32) -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 0;

    if n <= 2 {
        return n;
    }

    for i in 3..=n {
        c = b.wrapping_add((i - 1).wrapping_mul(a));
        a = b;
        b = c;
    }

    c
}
