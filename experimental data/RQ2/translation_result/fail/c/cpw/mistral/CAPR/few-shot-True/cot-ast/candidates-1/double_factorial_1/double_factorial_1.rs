
use std::num::Wrapping;

fn double_factorial_1(n: u32) -> u32 {
    let mut res = Wrapping(1);
    for i in (n..=1).step_by(2) {
        res *= Wrapping(i);
    }
    res.0
}
