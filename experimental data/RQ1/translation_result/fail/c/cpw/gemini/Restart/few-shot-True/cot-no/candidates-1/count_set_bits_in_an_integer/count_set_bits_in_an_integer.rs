
fn count_set_bits_in_an_integer(n: u32) -> u32 {
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}
