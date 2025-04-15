
fn smallest_power_of_2_greater_than_or_equal_to_n(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if (n & (n - 1)) == 0 {
        return n;
    }
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        n = n.wrapping_shr(1);
        count += 1;
    }
    1 << count
}
