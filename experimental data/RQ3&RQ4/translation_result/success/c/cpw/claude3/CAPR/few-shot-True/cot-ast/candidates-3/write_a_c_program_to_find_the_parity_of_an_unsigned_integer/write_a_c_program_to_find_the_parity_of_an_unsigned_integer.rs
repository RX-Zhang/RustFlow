
fn write_a_c_program_to_find_the_parity_of_an_unsigned_integer(mut n: u32) -> bool {
    let mut parity = false;
    while n != 0 {
        parity = !parity;
        n = n & (n.wrapping_sub(1));
    }
    parity
}
