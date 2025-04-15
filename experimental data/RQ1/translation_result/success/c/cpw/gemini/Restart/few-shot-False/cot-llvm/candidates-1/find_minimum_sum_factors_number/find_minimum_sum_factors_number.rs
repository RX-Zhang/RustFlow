
fn find_minimum_sum_factors_number(mut num: i32) -> i32 {
    let mut sum = 0;
    for i in 2..=(num as f64).sqrt() as i32 {
        while num % i == 0 {
            sum += i;
            num /= i;
        }
    }
    sum += num;
    sum
}
