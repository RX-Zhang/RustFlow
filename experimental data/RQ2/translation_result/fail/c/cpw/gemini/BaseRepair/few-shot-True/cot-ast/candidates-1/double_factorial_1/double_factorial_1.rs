
fn double_factorial_1(n: u32) -> u32 {
    let mut res = 1;
    for i in (0..=n).rev().step_by(2) {
        if i == 0 || i == 1 {
            return res;
        } else {
            res *= i;
        }
    }
    res // Return the result after the loop
}
