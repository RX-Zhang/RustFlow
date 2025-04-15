
use std::i32;

fn program_octaldecimal_conversion(n: i32) -> i32 {
    let num = n;
    let mut dec_value: i32 = 0; // specify i32 type for dec_value
    let mut base = 1;
    let mut temp = num;

    while temp != 0 {
        let last_digit = temp % 10;
        temp = temp / 10;
        dec_value = dec_value.wrapping_add(last_digit as i32 * base); // explicitly cast last_digit to i32
        base = base.wrapping_shl(3);
    }

    dec_value
}
