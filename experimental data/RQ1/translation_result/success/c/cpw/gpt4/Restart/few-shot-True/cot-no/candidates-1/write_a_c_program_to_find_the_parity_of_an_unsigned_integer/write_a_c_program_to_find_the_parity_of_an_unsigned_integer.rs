
fn write_a_c_program_to_find_the_parity_of_an_unsigned_integer(n: u32) -> i32 {
    let mut n = n;
    let mut parity = false;
    while n != 0 {
        parity = !parity;
        n = n & (n.wrapping_sub(1));
    }
    parity as i32
}
