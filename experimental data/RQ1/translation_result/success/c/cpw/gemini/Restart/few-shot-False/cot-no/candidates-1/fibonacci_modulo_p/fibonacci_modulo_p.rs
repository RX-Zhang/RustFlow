
fn fibonacci_modulo_p(p: i32) -> i32 {
    let mut first = 1;
    let mut second = 1;
    let mut number = 2;
    let mut next = 1;

    while next != 0 {
        next = (first + second) % p;
        first = second;
        second = next;
        number += 1;
    }
    number
}
