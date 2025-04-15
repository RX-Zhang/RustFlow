
fn smallest_power_of_2_greater_than_or_equal_to_n(mut n: u32) -> u32 {
    let mut count: u32 = 0;
    if n != 0 && n & (n.wrapping_sub(1)) == 0 {
        return n;
    }
    while n != 0 {
        n = n.wrapping_shr(1 % 32); 
        count = count.wrapping_add(1);
    }
    1u32.wrapping_shl(count % 32)
}
