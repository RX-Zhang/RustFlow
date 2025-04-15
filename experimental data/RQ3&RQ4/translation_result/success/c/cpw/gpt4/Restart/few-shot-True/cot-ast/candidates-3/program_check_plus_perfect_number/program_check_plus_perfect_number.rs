
fn program_check_plus_perfect_number(x: i32) -> bool {
    let mut temp = x;
    let mut n = 0;
    while temp != 0 {
        temp /= 10;
        n += 1;
    }
    temp = x;
    let mut sum: i32 = 0;
    let mut x = x;
    while x != 0 {
        let digit = (x % 10) as i32;
        sum = sum.wrapping_add(digit.wrapping_pow(n as u32));
        x /= 10;
    }
    sum == temp
}
