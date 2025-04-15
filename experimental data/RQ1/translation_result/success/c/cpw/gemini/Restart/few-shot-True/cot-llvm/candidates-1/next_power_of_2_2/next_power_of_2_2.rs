
#[inline(always)]
fn next_power_of_2_2(n: u32) -> u32 {
    let mut n = n;
    n = n.wrapping_sub(1);
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n.wrapping_add(1)
}
