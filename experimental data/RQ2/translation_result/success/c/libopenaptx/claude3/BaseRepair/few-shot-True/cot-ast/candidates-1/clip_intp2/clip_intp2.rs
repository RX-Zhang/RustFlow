use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    // Ensure p is within valid range for shifts
    let p = p % 32;
    
    let one_shifted = if p >= 31 { 0u32 } else { 1u32 << p };
    let two_shifted = if p >= 31 { 0u32 } else { 2u32 << p };
    let mask = two_shifted.wrapping_sub(1);
    
    if (Wrapping(a as u32) + Wrapping(one_shifted)) & !Wrapping(mask) != Wrapping(0) {
        let shift_result = if p >= 31 { (a >> 31) } else { (a >> 31) ^ ((1i32 << p) - 1) };
        shift_result
    } else {
        a
    }
}
