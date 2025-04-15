
fn write_a_c_program_to_find_the_parity_of_an_unsigned_integer(n: u32) -> bool {
    let mut parity = false;
    let mut n = n;
    while n != 0 {
        parity = !parity;
        n = n & (n - 1);
    }
    parity
}
