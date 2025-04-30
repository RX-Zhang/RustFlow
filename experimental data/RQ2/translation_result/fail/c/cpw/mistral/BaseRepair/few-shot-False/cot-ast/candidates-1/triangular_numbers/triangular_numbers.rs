

fn triangular_numbers(num: i32) -> bool {
    if num < 0 {
        return false;
    }
    let mut sum = 0;
    let mut n = 1;
    while sum <= num {
        sum = sum.wrapping_add(n);
        if sum == num {
            return true;
        }
        n = n.wrapping_add(1);
    }
    false
}
