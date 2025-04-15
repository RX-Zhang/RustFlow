
fn find_sum_odd_factors_number(mut n: i32) -> i32 {
    let mut res = 1;
    while n % 2 == 0 {
        n /= 2;
    }
    for i in (3..=(n as f64).sqrt() as i32).step_by(2) {
        let mut count = 0;
        let mut curr_sum = 1;
        let mut curr_term = 1;
        while n % i == 0 {
            count += 1;
            n /= i;
            curr_term *= i;
            curr_sum += curr_term;
        }
        res *= curr_sum;
    }
    if n >= 2 {
        res *= (1 + n);
    }
    res
}
