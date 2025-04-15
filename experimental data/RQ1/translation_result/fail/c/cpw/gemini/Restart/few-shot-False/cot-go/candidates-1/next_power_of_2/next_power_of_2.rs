
fn next_power_of_2(n: u32) -> u32 {
    if n != 0 && n & (n - 1) == 0 {
        return n;
    }

    let mut count = 0;
    let mut x = n;
    while x != 0 {
        x >>= 1;
        count += 1;
    }
    1 << count
}
