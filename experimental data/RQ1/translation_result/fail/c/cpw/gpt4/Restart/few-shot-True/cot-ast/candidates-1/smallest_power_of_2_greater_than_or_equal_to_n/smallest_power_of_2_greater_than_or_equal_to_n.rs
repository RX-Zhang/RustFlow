
fn smallest_power_of_2_greater_than_or_equal_to_n(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count: i32 = 0;
    if n != 0 && (n & (n - 1)) == 0 {
        return n;
    }
    let mut n = n;
    while n != 0 {
        n = n.wrapping_shr(1);
        count = count.wrapping_add(1);
    }
    1u32.wrapping_shl(count as u32)
}
