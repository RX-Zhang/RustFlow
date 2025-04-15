
fn composite_number(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    }
    if n <= 3 {
        return 0;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return 1;
    }
    let mut i: i32 = 5;
    while i.wrapping_mul(i) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return 1;
        }
        i = i.wrapping_add(6);
    }
    0
}
