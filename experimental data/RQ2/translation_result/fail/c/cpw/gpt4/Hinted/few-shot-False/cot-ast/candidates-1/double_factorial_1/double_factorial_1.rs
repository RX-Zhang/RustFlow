
fn double_factorial_1(n: u32) -> u32 {
    let mut res: u32 = 1;
    let mut i = n;

    while i > 1 {
        res = res.wrapping_mul(i);
        i = i.wrapping_sub(2);
    }

    res
}
