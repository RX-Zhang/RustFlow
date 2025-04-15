
fn nth_non_fibonacci_number(mut n: i32) -> i32 {
    let mut prev_prev: i32 = 1;
    let mut prev: i32 = 2;
    let mut curr: i32 = 3;

    while n > 0 {
        prev_prev = prev;
        prev = curr;
        curr = prev_prev.wrapping_add(prev);

        if curr <= prev {
            break;
        }

        let gap = curr.wrapping_sub(prev).wrapping_sub(1);
        if gap >= n {
            break;
        } else {
            n = n.wrapping_sub(gap);
        }
    }
    prev.wrapping_add(n)
}
