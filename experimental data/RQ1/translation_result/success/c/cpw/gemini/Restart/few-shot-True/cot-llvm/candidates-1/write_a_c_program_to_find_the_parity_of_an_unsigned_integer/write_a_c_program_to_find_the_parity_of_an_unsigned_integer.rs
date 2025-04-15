
fn find_parity_of_unsigned_integer(n: u32) -> bool {
    let mut parity = false;
    let mut num = n;
    while num != 0 {
        parity = !parity;
        num = num & (num - 1);
    }
    parity
}
