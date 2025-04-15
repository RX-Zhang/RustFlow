
fn composite_number(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return false;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return true;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return true;
        }
        i = i.wrapping_add(6);
    }
    false
}
