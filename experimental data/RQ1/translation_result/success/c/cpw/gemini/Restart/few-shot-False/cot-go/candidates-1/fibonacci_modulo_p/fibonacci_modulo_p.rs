
use std::num::Wrapping;

fn fibonacci_modulo_p(p: i32) -> i32 {
    let mut first: Wrapping<i32> = Wrapping(1);
    let mut second: Wrapping<i32> = Wrapping(1);
    let mut number: i32 = 2;
    let mut next: Wrapping<i32> = Wrapping(1);

    while next.0 != 0 {
        next = (first + second) % Wrapping(p);
        first = second;
        second = next;
        number += 1;
    }

    number
}
