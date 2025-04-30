
fn find_minimum_sum_factors_number(num: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut num = num;

    if num < 0 {
        return num; // Return the negative number as is
    }

    for i in 2..=((num as f64).sqrt() as i32) {
        while num % i == 0 {
            sum = sum.wrapping_add(i);
            num /= i;
        }
    }
    if num > 1 {
        sum = sum.wrapping_add(num);
    } else if num == 1 && sum == 0 {
        sum = 1; // Adjust for cases like input `4`
    }
    sum
}
