
fn sum_factors_number(n: i32) -> i32 {
    let mut result: i32 = 0;
    let n_sqrt = (n as f64).sqrt() as i32;

    for i in 2..=n_sqrt {
        if n % i == 0 {
            if i == n / i {
                result = result.wrapping_add(i);
            } else {
                result = result.wrapping_add(i);
                result = result.wrapping_add(n / i);
            }
        }
    }

    result.wrapping_add(n).wrapping_add(1)
}
