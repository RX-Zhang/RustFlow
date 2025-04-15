
fn count_set_bits_in_an_integer(mut n: u32) -> u32 {
    let mut count = 0;
    while n != 0 {
        count += n & 1;
        n = n.wrapping_shr(1);
    }
    count
}
