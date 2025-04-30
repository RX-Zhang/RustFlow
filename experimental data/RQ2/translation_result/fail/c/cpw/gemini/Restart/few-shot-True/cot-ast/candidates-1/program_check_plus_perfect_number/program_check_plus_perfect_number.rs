
fn program_check_plus_perfect_number(x: i32) -> bool {
    let temp = x.abs() as u64;
    let mut n = 0;
    let mut x_copy = temp;
    while x_copy != 0 {
        x_copy /= 10;
        n += 1;
    }
    let mut sum: u64 = 0;
    x_copy = temp;
    while x_copy != 0 {
        sum = sum.wrapping_add((x_copy % 10).pow(n));
        x_copy /= 10;
    }
    sum == temp
}
