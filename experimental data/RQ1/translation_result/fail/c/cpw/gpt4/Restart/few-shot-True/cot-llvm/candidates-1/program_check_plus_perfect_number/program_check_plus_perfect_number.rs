
use std::num::Wrapping;

fn program_check_plus_perfect_number(mut x: i32) -> bool {
    let temp = x;
    let mut n = 0;
    while x != 0 {
        x = x.wrapping_div(10);
        n += 1;
    }
    x = temp;
    let mut sum = Wrapping(0);
    while x != 0 {
        sum += Wrapping((x % 10).pow(n as u32));
        x /= 10;
    }
    sum.0 == temp
}
