
fn count_total_set_bits_in_all_numbers_from_1_to_n(n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut ans: i32 = 0;
    
    while (1_i32).wrapping_shl(i as u32) <= n {
        let mut k = false;
        let mut change = (1_i32).wrapping_shl(i as u32);
        
        for _j in 0..=n {
            if k {
                ans = ans.wrapping_add(1);
            }
            
            if change == 1 {
                k = !k;
                change = (1_i32).wrapping_shl(i as u32);
            } else {
                change = change.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }
    ans
}
