fn sum_of_all_proper_divisors_of_a_natural_number(num: i32) -> i32 {
    let mut result: i32 = 0;
    let sqrt_num = (num as f64).sqrt() as i32;
    for i in 2..=sqrt_num {
        if num % i == 0 {
            if i == num / i {
                result = result.wrapping_add(i);
            } else {
                result = result.wrapping_add(i).wrapping_add(num / i);
            }
        }
    }
    result.wrapping_add(1)
}