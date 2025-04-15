
fn find_whether_a_given_number_is_a_power_of_4_or_not_1(n: u32) -> bool {
    let mut count = 0;
    let mut n = n;
    if n != 0 && (n & (n - 1)) == 0 {
        while n > 1 {
            n = n.wrapping_shr(1);
            count += 1;
        }
        return count % 2 == 0;
    }
    false
}
