
fn finding_power_prime_number_p_n_1(n: i32, p: i32) -> i32 {
    let mut ans = 0;
    let mut temp = p;
    while temp <= n {
        ans += n / temp;
        temp = temp.wrapping_mul(p);
    }
    ans
}
