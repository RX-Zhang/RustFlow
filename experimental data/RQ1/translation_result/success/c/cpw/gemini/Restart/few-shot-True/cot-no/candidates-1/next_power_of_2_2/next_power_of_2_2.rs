
fn next_power_of_2_2(n: u32) -> u32 {
    let mut n = n.wrapping_sub(1);
    n = n | (n >> 1);
    n = n | (n >> 2);
    n = n | (n >> 4);
    n = n | (n >> 8);
    n = n | (n >> 16);
    n.wrapping_add(1)
}
