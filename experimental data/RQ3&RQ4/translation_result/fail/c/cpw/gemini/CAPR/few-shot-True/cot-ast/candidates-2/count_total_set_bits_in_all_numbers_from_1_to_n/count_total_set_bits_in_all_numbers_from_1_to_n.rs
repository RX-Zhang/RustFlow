
fn count_total_set_bits_in_all_numbers_from_1_to_n(n: i32) -> i32 {
    let mut i = 0;
    let mut ans = 0;
    while (1 << i) <= n {
        let mut k = false;
        let mut change = 1 << i;
        for j in 0..=n {
            ans += k as i32;
            if change == 1 {
                k = !k;
                change = 1 << i;
            } else {
                change -= 1;
            }
        }
        i += 1;
    }
    ans
}
