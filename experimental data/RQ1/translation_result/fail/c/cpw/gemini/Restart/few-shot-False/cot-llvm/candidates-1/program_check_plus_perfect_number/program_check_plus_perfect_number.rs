
fn program_check_plus_perfect_number(x: i32) -> bool {
    let mut temp = x;
    let mut n = 0;
    while temp != 0 {
        temp /= 10;
        n += 1;
    }
    let mut sum = 0;
    temp = x;
    while temp != 0 {
        sum += (temp % 10).pow(n as u32);
        temp /= 10;
    }
    sum == x
}
