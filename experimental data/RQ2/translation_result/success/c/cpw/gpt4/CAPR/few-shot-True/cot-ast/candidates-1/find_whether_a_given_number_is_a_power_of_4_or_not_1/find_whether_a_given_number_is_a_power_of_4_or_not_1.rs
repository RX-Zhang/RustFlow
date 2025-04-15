
fn find_whether_a_given_number_is_a_power_of_4_or_not_1(n: u32) -> i32 {
    let mut count: i32 = 0;
    if n != 0 && (n & (n - 1)) == 0 {
        let mut n = n;
        while n > 1 {
            n = n.wrapping_shr(1);
            count = count.wrapping_add(1);
        }
        return if count % 2 == 0 { 1 } else { 0 };
    }
    0
}
