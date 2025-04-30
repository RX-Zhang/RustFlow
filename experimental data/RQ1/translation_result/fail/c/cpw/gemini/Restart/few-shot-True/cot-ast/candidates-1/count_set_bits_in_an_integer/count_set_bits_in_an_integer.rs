
fn count_set_bits_in_an_integer(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut n = n;
    while n != 0 {
        count = count.wrapping_add(n & 1);
        n = n.wrapping_shr(1);
    }
    count
}
