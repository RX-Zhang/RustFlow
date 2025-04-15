
fn write_a_c_program_to_find_the_parity_of_an_unsigned_integer(n: u32) -> bool {
    let mut parity = false;
    let mut n_mut = n;
    while n_mut != 0 {
        parity = !parity;
        n_mut = n_mut & (n_mut.wrapping_sub(1));
    }
    parity
}
