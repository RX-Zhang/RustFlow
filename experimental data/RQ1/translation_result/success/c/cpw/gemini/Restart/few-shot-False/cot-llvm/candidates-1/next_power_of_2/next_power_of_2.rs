
fn next_power_of_2(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if n & (n - 1) == 0 {
        return n;
    }
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        n >>= 1;
        count += 1;
    }
    1 << count
}
