
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
    for i in (5..=(n as f64).sqrt() as i32).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return true;
        }
    }
    false
}
