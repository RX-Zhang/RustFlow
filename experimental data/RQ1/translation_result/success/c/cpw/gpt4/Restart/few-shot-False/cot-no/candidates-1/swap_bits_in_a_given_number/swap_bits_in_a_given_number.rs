fn swap_bits_in_a_given_number(x: u32, p1: u32, p2: u32, n: u32) -> u32 {
    let set1 = (x.wrapping_shr(p1)) & ((1u32.wrapping_shl(n)).wrapping_sub(1));
    let set2 = (x.wrapping_shr(p2)) & ((1u32.wrapping_shl(n)).wrapping_sub(1));
    let xor = set1 ^ set2;
    let xor_shifted = (xor.wrapping_shl(p1)) | (xor.wrapping_shl(p2));
    let result = x ^ xor_shifted;
    result
}