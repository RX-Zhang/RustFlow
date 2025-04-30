fn next_power_of_2(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut n_mut = n;
    if n != 0 && (n & (n.wrapping_sub(1))) == 0 {
        return n;
    }
    while n_mut != 0 {
        n_mut = n_mut.wrapping_shr(1);
        count = count.wrapping_add(1);
    }
    1u32.wrapping_shl(count)
}