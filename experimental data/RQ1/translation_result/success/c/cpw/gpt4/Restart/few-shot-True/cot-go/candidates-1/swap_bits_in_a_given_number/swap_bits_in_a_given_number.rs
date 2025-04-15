
fn swap_bits_in_a_given_number(x: u32, p1: u32, p2: u32, n: u32) -> u32 {
    let set1 = (x >> (p1 % 32)) & ((1u32.wrapping_shl(n)) - 1);
    let set2 = (x >> (p2 % 32)) & ((1u32.wrapping_shl(n)) - 1);
    let xor = set1 ^ set2;
    let xor_shifted = (xor.wrapping_shl(p1 % 32)) | (xor.wrapping_shl(p2 % 32));
    x ^ xor_shifted
}
