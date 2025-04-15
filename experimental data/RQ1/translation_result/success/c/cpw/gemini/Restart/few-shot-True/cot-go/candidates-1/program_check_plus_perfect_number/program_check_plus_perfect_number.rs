
fn program_check_plus_perfect_number(x: i32) -> bool {
    let mut temp = x;
    let mut n = 0;
    while temp != 0 {
        temp = temp.wrapping_div(10);
        n += 1;
    }
    temp = x;
    let mut sum = 0i64; // Using i64 to avoid overflow
    while temp != 0 {
        let digit = (temp % 10) as i64; // Using i64 to avoid overflow
        sum = sum.wrapping_add(digit.pow(n as u32));
        temp = temp.wrapping_div(10);
    }
    sum == x as i64 // Compare with x as i64 to avoid overflow
}
