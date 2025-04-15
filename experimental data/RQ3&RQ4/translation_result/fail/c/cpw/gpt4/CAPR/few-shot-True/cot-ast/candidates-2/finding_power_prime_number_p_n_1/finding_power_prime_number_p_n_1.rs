
fn finding_power_prime_number_p_n_1(n: i32, p: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut temp: i32 = p;
    while temp <= n {
        ans = ans.wrapping_add(n / temp);
        temp = temp.wrapping_mul(p);
    }
    ans
}
