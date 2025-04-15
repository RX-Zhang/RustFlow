
use std::num::Wrapping;

fn finding_power_prime_number_p_n(n: i32, p: i32) -> i32 {
    let mut ans = Wrapping(0);
    let mut temp = Wrapping(p);
    while temp <= Wrapping(n) {
        ans += Wrapping(n) / temp;
        temp = temp * Wrapping(p);
    }
    ans.0
}
