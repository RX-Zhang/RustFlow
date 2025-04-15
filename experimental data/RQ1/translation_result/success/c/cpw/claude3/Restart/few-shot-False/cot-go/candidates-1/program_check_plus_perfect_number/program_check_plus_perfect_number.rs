
fn program_check_plus_perfect_number(x: i32) -> bool {
    let temp = x;
    let mut n = 0;
    let mut x_copy = x;

    while x_copy != 0 {
        x_copy /= 10;
        n += 1;
    }

    x_copy = temp;
    let mut sum = 0i64;  // Use i64 to prevent overflow

    while x_copy != 0 {
        let digit = x_copy % 10;
        sum = sum.wrapping_add((digit as i64).wrapping_pow(n as u32));
        x_copy /= 10;
    }

    sum == temp as i64
}
