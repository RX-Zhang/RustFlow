
fn nth_non_fibonacci_number(n: i32) -> i32 {
    let mut n = n;
    let mut prev_prev: i32 = 1;
    let mut prev: i32 = 2;
    let mut curr: i32 = 3;

    while n > 0 {
        prev_prev = prev;
        prev = curr;
        curr = prev_prev.wrapping_add(prev);
        n = n.wrapping_sub(curr.wrapping_sub(prev).wrapping_sub(1));
    }

    n = n.wrapping_add(curr.wrapping_sub(prev).wrapping_sub(1));
    prev.wrapping_add(n)
}
