
use std::ops::Shr;
use std::usize;

fn minimum_sum_factorsnumber(num: u32) -> u32 {
    let mut sum: u32 = 0; // specify the type of sum as u32
    let mut n = num;
    for i in 2.. {
        let factor_count = (n as f64).log(i as f64).floor() as usize;
        if factor_count == 0 {
            break;
        }
        sum = sum.wrapping_add((i as u32).wrapping_shl(factor_count as u32));
        n = n.shr(factor_count);
    }
    sum.wrapping_add(n) // remove the cast as u32, since n is already a u32
}
