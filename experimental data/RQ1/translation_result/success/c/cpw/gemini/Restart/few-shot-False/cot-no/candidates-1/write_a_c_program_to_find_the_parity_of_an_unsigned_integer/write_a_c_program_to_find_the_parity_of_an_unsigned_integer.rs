

fn write_a_c_program_to_find_the_parity_of_an_unsigned_integer(n: u32) -> bool {
    let mut n = n;
    let mut parity = false;

    while n > 0 {
        parity = !parity;
        n = n.wrapping_shr(1).wrapping_sub(1) & n;
    }

    parity
}
