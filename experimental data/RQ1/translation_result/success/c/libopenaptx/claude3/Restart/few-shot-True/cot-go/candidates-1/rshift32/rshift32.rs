use std::num::Wrapping;

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift.wrapping_sub(1) % 32);
    let mask = (1i32 << (shift.wrapping_add(1) % 32)).wrapping_sub(1);
    let result = ((Wrapping(value) + Wrapping(rounding)).0 >> (shift % 32)).wrapping_sub(
        ((value & mask) == rounding) as i32
    );
    result
}
