
fn padovan_sequence(n: i32) -> i32 {
    let mut p_prev_prev: i32 = 1;
    let mut p_prev: i32 = 1;
    let mut p_curr: i32 = 1;
    let mut p_next: i32 = 1;

    for _ in 3..=n {
        p_next = p_prev_prev.wrapping_add(p_prev);
        p_prev_prev = p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }
    p_next
}
