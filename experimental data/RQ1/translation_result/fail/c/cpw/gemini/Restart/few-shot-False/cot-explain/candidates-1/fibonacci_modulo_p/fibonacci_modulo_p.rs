
fn fibonacci_modulo_p(p: i32) -> i32 {
    let mut first = 1i32;
    let mut second = 1i32;
    let mut number = 2i32;
    let mut next = 1i32;
    while next != 0 {
        next = first.wrapping_add(second).wrapping_rem(p);
        first = second;
        second = next;
        number = number.wrapping_add(1);
    }
    number
}
