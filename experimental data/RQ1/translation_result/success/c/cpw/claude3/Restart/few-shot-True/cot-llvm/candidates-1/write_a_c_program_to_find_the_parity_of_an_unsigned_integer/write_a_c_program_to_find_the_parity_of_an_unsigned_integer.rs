
fn write_a_c_program_to_find_the_parity_of_an_unsigned_integer(n: u32) -> bool {
    let mut parity = false;
    let mut num = n;
    while num != 0 {
        parity = !parity;
        num = num & (num.wrapping_sub(1));
    }
    parity
}
