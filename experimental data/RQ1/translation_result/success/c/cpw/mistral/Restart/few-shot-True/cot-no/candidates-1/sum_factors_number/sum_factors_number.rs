
fn sum_factors_number(n: i32) -> i32 {
    let mut result = 0;
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            if i == n / i {
                result += i;
            } else {
                result += i + n / i;
            }
        }
    }
    result + n + 1
}
