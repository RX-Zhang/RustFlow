
fn find_whether_a_given_number_is_a_power_of_4_or_not_1(n: u32) -> i32 {
    let mut count: i32 = 0;
    let mut num = n;

    if num != 0 && num & (num.wrapping_sub(1)) == 0 {
        while num > 1 {
            num = num.wrapping_shr(1 % 32);
            count = count.wrapping_add(1);
        }
        if count % 2 == 0 { 1 } else { 0 }
    } else {
        0
    }
}
