
fn program_binary_decimal_conversion(n: i32) -> i32 {
    let mut num = n;
    let mut dec_value: i32 = 0;
    let mut base: i32 = 1;
    let mut temp = num;

    while temp != 0 {
        let last_digit = temp.wrapping_rem(10);
        temp = temp.wrapping_div(10); 
        dec_value = dec_value.wrapping_add(last_digit.wrapping_mul(base));
        base = base.wrapping_mul(2);
    }
    dec_value
}
