
fn padovan_sequence(n: i32) -> i32 {
    let mut p_prev_prev = 1;
    let mut p_prev = 1;
    let mut p_curr = 1;
    let mut p_next = 1;
    for i in 3..=n {
        p_next = p_prev_prev + p_prev;
        p_prev_prev = p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }
    p_next
}
