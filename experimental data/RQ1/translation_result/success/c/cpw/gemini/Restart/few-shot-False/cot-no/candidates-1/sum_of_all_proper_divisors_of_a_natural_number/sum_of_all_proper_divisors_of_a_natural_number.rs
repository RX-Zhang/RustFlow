
fn sum_of_all_proper_divisors_of_a_natural_number(num: i32) -> i32 {
    let mut result = 0;
    for i in 2..=(num as f64).sqrt() as i32 {
        if num % i == 0 {
            if i == num / i {
                result += i;
            } else {
                result += i + num / i;
            }
        }
    }
    result + 1
}
