
fn program_octal_decimal_conversion(n: i32) -> i32 {
    let mut num = n;
    let mut dec_value: i32 = 0;
    let mut base: i32 = 1;

    while num != 0 {
        let last_digit = num.wrapping_rem(10);
        num = num.wrapping_div(10);
        dec_value = dec_value.wrapping_add(last_digit.wrapping_mul(base));
        base = base.wrapping_mul(8);
    }

    dec_value
}
