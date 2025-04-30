
use std::u32;

fn swap_bits_in_a_given_number(x: u32, p1: u32, p2: u32, n: u32) -> u32 {
    let set1 = (x >> p1) & ((1u32 << n) - 1);
    let set2 = (x >> p2) & ((1u32 << n) - 1);
    let xor = set1.wrapping_add(set2.wrapping_neg());
    let xor_shifted = ((xor as i32).wrapping_shl(p1 as u32) as u32) | ((xor as i32).wrapping_shl(p2 as u32) as u32);
    x.wrapping_add(xor_shifted.wrapping_neg())
}
