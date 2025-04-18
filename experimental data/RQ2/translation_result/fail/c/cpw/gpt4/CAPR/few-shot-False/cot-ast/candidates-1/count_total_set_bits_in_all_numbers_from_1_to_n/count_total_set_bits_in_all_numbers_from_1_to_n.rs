
fn count_total_set_bits_in_all_numbers_from_1_to_n(n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut ans: i32 = 0;
    while (1 << i) <= n {
        let mut k = false;
        let mut change: i32 = 1 << i;
        let mut j: i32 = 0
        while j <= n {
            ans = ans.wrapping_add(k as i32);
            if change == 1 {
                k = !k;
                change = 1 << i;
            } else {
                change = change.wrapping_sub(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    ans
}
