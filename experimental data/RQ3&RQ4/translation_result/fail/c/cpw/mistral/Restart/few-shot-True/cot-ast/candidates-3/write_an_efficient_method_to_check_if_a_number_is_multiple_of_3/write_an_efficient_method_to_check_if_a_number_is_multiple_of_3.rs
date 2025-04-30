
use std::cmp::Ordering;
use std::i32;

fn is_multiple_of_three(n: i32) -> bool {
    fn helper(mut n: i32) -> Ordering {
        if n < 0 {
            n = -n;
        }
        if n == 0 {
            Ordering::Equal
        } else if n == 1 {
            Ordering::Less
        } else {
            let mut odd_count = 0;
            let mut even_count = 0;
            while n > 0 {
                if n & 1 == 1 {
                    odd_count += 1;
                }
                if n & 2 == 0 {
                    even_count += 1;
                }
                n = n.wrapping_shr(2);
            }
            helper(n.wrapping_sub(odd_count as i32).wrapping_sub(even_count as i32))
        }
    }
    helper(n.abs()) == Ordering::Equal
}
