fn find_minimum_sum_factors_number(num: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut num_mut = num;

    let mut i: i32 = 2;
    while i.wrapping_mul(i) <= num_mut {
        while num_mut.wrapping_rem(i) == 0 {
            sum = sum.wrapping_add(i);
            num_mut = num_mut.wrapping_div(i);
        }
        i = i.wrapping_add(1);
    }

    sum = sum.wrapping_add(num_mut);
    sum
}