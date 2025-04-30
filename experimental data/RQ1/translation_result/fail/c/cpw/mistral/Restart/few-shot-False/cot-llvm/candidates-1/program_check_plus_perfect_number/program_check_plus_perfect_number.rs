
use std::cmp::min;
use std::usize;
use std::convert::TryInto;
use std::ops::Shr;

fn program_check_plus_perfect_number(x: i32) -> bool {
    let temp = x;
    let n = num_digits(x);
    let mut sum = 0;
    let mut x = temp;

    while x != 0 {
        sum += digit_power(x.rem_euclid(10), n);
        x = x.shr(1);
    }

    sum == temp
}

fn num_digits(num: i32) -> usize {
    if num < 0 {
        (-num).abs() as usize
    } else {
        num as usize
    }
}

fn digit_power(digit: i32, power: usize) -> i32 {
    (digit as u32).pow(min(power as u32, 32u32)) as i32
}
