
fn swap_bits_in_a_given_number(x: u32, p1: u32, p2: u32, n: u32) -> u32 {
    let n_mask = (1u32.wrapping_shl(n as u32)).wrapping_sub(1);
    let set1 = (x.wrapping_shr(p1 as u32)) & n_mask;
    let set2 = (x.wrapping_shr(p2 as u32)) & n_mask;
    let xor_result = set1 ^ set2;
    let xor = (xor_result.wrapping_shl(p1 as u32)) | (xor_result.wrapping_shl(p2 as u32));
    x ^ xor
}
