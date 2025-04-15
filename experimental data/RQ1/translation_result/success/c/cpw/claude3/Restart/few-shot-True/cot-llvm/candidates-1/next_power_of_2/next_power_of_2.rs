
fn next_power_of_2(mut n: u32) -> u32 {
    let mut count: u32 = 0;
    if n != 0 && n & (n.wrapping_sub(1)) == 0 {
        return n;
    }
    while n != 0 {
        n = n.wrapping_shr(1);
        count = count.wrapping_add(1);
    }
    1u32.wrapping_shl(count)
}
