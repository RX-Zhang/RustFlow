
use std::cmp::Ordering;
use std::ops::Shr;

fn square_root_of_an_integer_1(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut start = 1;
    let mut end = x;
    let mut ans = 0;

    while start <= end {
        let mid: i32 = start.wrapping_add(end).shr(1);

        match mid.pow(2).cmp(&x) {
            Ordering::Equal => {
                return mid;
            }
            Ordering::Less => {
                start = mid.wrapping_add(1);
                ans = mid;
            }
            Ordering::Greater => {
                end = mid.saturating_sub(1);
            }
        }
    }

    ans
}
