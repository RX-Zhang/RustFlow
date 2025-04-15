use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_wrapped = Wrapping(a as u32);
    let one = Wrapping(1u32);
    let two = Wrapping(2u32);
    
    if (a_wrapped + (one << (p as usize))) & !((two << (p as usize)) - one) != Wrapping(0) {
        ((a >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift.wrapping_sub(1) % 64);
    let mask = (1i64 << (shift.wrapping_add(1) % 64)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> (shift % 64)).wrapping_sub(((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
