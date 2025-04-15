
fn find_minimum_sum_factors_number(num: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut num = num;
    for i in 2..=((num as f64).sqrt() as i32) {
        while num % i == 0 {
            sum = sum.wrapping_add(i);
            num /= i;
        }
    }
    if num > 1 {
        sum = sum.wrapping_add(num);
    }
    sum
}
