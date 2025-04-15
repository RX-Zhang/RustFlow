
fn count_set_bits_in_an_integer(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut num = n;
    while num > 0 {
        count = count.wrapping_add(num & 1);
        num = num.wrapping_shr(1);
    }
    count
}
