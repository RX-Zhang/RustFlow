
fn next_power_of_2(mut n: u32) -> u32 {
    let mut count = 0;
    if n != 0 && (n & (n - 1)) == 0 {
        return n;
    }
    while n != 0 {
        n = n.wrapping_shr(1);
        count += 1;
    }
    1u32.wrapping_shl(count as u32)
}
