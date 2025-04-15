
fn next_power_of_2(n: u32) -> u32 {
    let mut count: u32 = 0; // Specify explicit type for count
    if n != 0 && (n & (n - 1)) == 0 {
        return n;
    }
    let mut temp_n = n;
    while temp_n != 0 {
        temp_n = temp_n.wrapping_shr(1);
        count = count.wrapping_add(1);
    }
    if count >= 32 {
        1
    } else {
        1 << count
    }
}
