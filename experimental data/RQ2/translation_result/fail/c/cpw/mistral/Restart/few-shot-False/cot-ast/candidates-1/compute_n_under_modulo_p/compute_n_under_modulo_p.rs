
use std::u32;
use std::i32;
use std::num::Wrapping;

fn compute_n_under_modulo_p(n: i32, p: u32) -> i32 {
    if n >= p as i32 {
        return 0;
    }
    let mut result = Wrapping(1);
    for i in 1..=n {
        result *= Wrapping(i);
        result %= Wrapping(p as i32);
    }
    result.0
}
