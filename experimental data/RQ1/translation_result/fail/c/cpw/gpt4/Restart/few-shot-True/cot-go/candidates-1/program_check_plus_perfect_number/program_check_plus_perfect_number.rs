
fn program_check_plus_perfect_number(x: i32) -> bool {
    let mut temp = x.abs();
    let mut n: u32 = 0;
    while temp != 0 {
        temp /= 10;
        n = n.wrapping_add(1);
    }
    temp = x.abs();
    let mut sum: i32 = 0;
    while temp != 0 {
        let digit = temp % 10;
        let power = Box::new(digit as u32).pow(n);
        sum = sum.wrapping_add(power as i32);
        temp /= 10;
    }
    sum == x.abs()
}
