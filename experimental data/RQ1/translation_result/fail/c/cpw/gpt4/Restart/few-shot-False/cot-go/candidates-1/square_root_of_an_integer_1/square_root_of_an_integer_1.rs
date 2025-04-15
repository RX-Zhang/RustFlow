
use std::cmp::Ordering;

fn square_root_of_an_integer_1(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }
    
    let mut start = 1;
    let mut end = x;
    let mut ans = 0;

    while start <= end {
        let mid = start + (end - start) / 2; // Avoid potential overflow

        match (mid * mid).cmp(&x) {
            Ordering::Equal => return mid,
            Ordering::Less => {
                start = mid + 1;
                ans = mid;
            },
            Ordering::Greater => end = mid - 1,
        }
    }

    ans
}
