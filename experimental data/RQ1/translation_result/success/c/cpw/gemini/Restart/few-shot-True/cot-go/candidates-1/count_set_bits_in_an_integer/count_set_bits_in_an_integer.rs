

fn count_set_bits_in_an_integer(n: u32) -> u32 {
    let mut count = 0;
    let mut num = n;
    while num != 0 {
        count += num & 1;
        num = num.wrapping_shr(1);
    }
    count
}
