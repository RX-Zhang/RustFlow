
use std::usize;

fn finding_power_prime_number_pn(n: i32, p: i32) -> i32 {
    let mut ans = 0;
    let mut temp = p;
    while temp <= n {
        ans += n as usize / temp as usize;
        temp = temp * p;
    }
    ans as i32
}
