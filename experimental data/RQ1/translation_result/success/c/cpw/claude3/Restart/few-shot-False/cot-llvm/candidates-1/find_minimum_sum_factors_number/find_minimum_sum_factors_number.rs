
fn find_minimum_sum_factors_number(mut num: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 2;

    while i.wrapping_mul(i) <= num {
        while num % i == 0 {
            sum = sum.wrapping_add(i);
            num /= i;
        }
        i = i.wrapping_add(1);
    }

    sum.wrapping_add(num)
}
