
fn smallest_power_of_2_greater_than_or_equal_to_n(mut n: u32) -> u32 {
    if n != 0 && (n & (n - 1)) == 0 {
        return n;
    }
    let mut count = 0;
    while n != 0 {
        n >>= 1;
        count += 1;
    }
    1 << count
}
