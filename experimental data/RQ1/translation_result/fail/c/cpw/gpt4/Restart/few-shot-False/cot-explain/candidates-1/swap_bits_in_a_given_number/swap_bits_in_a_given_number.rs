
fn swap_bits_in_a_given_number(x: u32, p1: u32, p2: u32, n: u32) -> u32 {
    let safe_n = n % 32;
    let mask = (1u32.wrapping_shl(safe_n) - 1) as u32;

    let set1 = (x.wrapping_shr(p1 % 32)) & mask;
    let set2 = (x.wrapping_shr(p2 % 32)) & mask;

    let xor = set1 ^ set2;
    let xor_shifted = (xor.wrapping_shl(p1 % 32)) | (xor.wrapping_shl(p2 % 32));

    x ^ xor_shifted
}
