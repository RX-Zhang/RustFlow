
use std::num::Wrapping;

#[inline]
pub fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_wrapped = Wrapping(a as u32);
    let one = Wrapping(1u32);
    let two = Wrapping(2u32);

    if (a_wrapped + (one << (p as usize))) & !((two << (p as usize)) - one) != Wrapping(0) {
        ((a >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a
    }
}
