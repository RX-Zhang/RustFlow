
fn double_factorial_1(n: u32) -> u32 {
    let mut res = 1;
    for i in (0..=n).rev().step_by(2) {
        if i == 0 || i == 1 {
            return res;
        } else {
            res = res.wrapping_mul(i as u32)
        }
    }
    res
}
