fn nth_non_fibonacci_number(n: i32) -> i32 {
    let mut prev_prev: i32 = 1;
    let mut prev: i32 = 2;
    let mut curr: i32 = 3;
    let mut n_mut: i32 = n;
    while n_mut > 0 {
        prev_prev = prev;
        prev = curr;
        curr = prev_prev.wrapping_add(prev);
        n_mut = n_mut.wrapping_sub(curr.wrapping_sub(prev).wrapping_sub(1));
    }
    n_mut = n_mut.wrapping_add(curr.wrapping_sub(prev).wrapping_sub(1));
    prev.wrapping_add(n_mut)
}