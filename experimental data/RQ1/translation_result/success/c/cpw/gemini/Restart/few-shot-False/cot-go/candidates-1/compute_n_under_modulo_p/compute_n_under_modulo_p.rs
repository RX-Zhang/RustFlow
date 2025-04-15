
use std::num::Wrapping;

fn compute_n_under_modulo_p(n: i32, p: i32) -> i32 {
    if n >= p {
        return 0;
    }

    let mut result: Wrapping<i32> = Wrapping(1);
    for i in 1..=n {
        result = (result * Wrapping(i)) % Wrapping(p);
    }
    result.0
}
