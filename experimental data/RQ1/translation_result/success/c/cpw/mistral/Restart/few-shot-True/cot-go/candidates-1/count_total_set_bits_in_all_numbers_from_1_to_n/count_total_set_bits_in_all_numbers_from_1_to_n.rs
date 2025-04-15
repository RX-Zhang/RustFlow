
use std::u32;

fn count_total_set_bits_in_all_numbers_up_to_n(n: u32) -> u32 {
    let mut i = 0;
    let mut ans = 0;

    while (1u32 << i) <= n {
        let mut k = false;
        let mut change = 1u32 << i;

        for j in 0..=n {
            ans = match u32::wrapping_add(ans, k as u32) {
                x if x > u32::MAX => {
                    return u32::MAX;
                }
                x => x,
            };

            if change == 1 {
                k = !k;
                change = 1u32 << i;
            } else {
                change = change.wrapping_sub(1);
            }
        }

        i += 1;
    }

    ans
}
