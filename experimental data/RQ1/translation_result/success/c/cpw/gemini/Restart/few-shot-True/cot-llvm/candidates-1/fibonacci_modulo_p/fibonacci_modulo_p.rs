
fn fibonacci_modulo_p(p: i32) -> i32 {
    let mut first: i32 = 1;
    let mut second: i32 = 1;
    let mut number: i32 = 2;
    let mut next: i32 = 1;

    while next != 0 {
        next = first.wrapping_add(second).wrapping_rem(p);
        first = second;
        second = next;
        number = number.wrapping_add(1);
    }

    number
}
