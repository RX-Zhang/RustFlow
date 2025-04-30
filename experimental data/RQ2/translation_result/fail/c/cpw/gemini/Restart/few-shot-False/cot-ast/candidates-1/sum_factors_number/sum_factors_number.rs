
fn sum_factors_number(n: i32) -> i32 {
    let mut result: i64 = 0;
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            if i == n / i {
                result += i as i64;
            } else {
                result += i as i64 + (n / i) as i64;
            }
        }
    }
    result += n as i64 + 1;
    result as i32
}
