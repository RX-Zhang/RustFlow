fn write_a_c_program_to_find_the_parity_of_an_unsigned_integer(n: u32) -> i32 {
    let mut parity: bool = false;
    let mut num: u32 = n;
    while num != 0 {
        parity = !parity;
        num = num & (num.wrapping_sub(1));
    }
    if parity { 1 } else { 0 }
}