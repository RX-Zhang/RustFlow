
use std::ops::*;
use std::usize;

fn dynamic_programming_set_36_cut_a_rope_to_maximize_product_1(n: i32) -> i32 {
    if n == 2 || n == 3 {
        return n - 1;
    }
    let mut res: i32 = 1;
    let mut n = n as usize;
    while n > 4 {
        n = n.wrapping_sub(3);
        res = res.wrapping_mul(3);
    }
    return (n as i32) * res;
}
