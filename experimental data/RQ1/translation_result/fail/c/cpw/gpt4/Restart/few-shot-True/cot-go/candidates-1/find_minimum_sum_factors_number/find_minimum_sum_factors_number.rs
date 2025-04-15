
fn find_minimum_sum_factors_number(mut num: i32) -> i32 {
    if num < 0 {
        return num;
    }

    let mut sum: i32 = 0;
    let mut i = 2;

    while i * i <= num {
        while num % i == 0 {
            sum = sum.wrapping_add(i);
            num /= i;
        }
        i += 1;
    }

    if num > 1 {
        sum = sum.wrapping_add(num);
    }

    sum
}
