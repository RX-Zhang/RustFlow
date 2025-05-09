
fn program_binary_decimal_conversion(n: i32) -> i32 {
    let num = n;
    let mut dec_value: i32 = 0;
    let mut base: i32 = 1;
    let mut temp = num;

    while temp != 0 {
        let last_digit = temp % 10;
        temp /= 10;
        dec_value = dec_value.wrapping_add(last_digit * base);
        base = base.wrapping_mul(2);
    }
    
    dec_value
}
