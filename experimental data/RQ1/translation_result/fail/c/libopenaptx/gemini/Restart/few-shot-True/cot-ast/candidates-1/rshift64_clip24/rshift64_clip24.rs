use std::convert::TryInto;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let uint_a: u32 = a.try_into().unwrap();
    let shifted_a = uint_a + (1 << p);
    if (shifted_a) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding: i64 = (1 << (shift - 1)) as i64;
    let mask: i64 = ((1 << (shift + 1)) - 1) as i64;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}
