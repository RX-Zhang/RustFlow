
use std::num::Wrapping;

fn padovan_sequence(n: i32) -> i32 {
    let mut p_prev_prev = Wrapping(1);
    let mut p_prev = Wrapping(1);
    let mut p_curr = Wrapping(1);
    let mut p_next = Wrapping(1);

    for _ in 3..=n {
        p_next = p_prev_prev + p_prev;
        p_prev_prev = p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_next.0
}
