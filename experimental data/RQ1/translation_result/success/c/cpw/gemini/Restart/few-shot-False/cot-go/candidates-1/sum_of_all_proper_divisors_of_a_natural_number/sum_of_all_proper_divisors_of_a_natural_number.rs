
fn sum_of_all_proper_divisors_of_a_natural_number(num: i32) -> i32 {
    let mut result: i64 = 0;
    for i in 2..=(num as f64).sqrt() as i32 {
        if num % i == 0 {
            if i == num / i {
                result = result.wrapping_add(i as i64);
            } else {
                result = result.wrapping_add(i as i64);
                result = result.wrapping_add((num / i) as i64);
            }
        }
    }
    let final_result = result.wrapping_add(1);
    final_result as i32
}
