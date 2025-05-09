
fn next_power_of_2_2(mut n: u32) -> u32 {
    n = n.wrapping_sub(1);
    n |= n.wrapping_shr(1);
    n |= n.wrapping_shr(2);
    n |= n.wrapping_shr(4);
    n |= n.wrapping_shr(8);
    n |= n.wrapping_shr(16);
    n = n.wrapping_add(1);
    n
}
