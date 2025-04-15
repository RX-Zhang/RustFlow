

fn triangular_numbers(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    let mut sum: i32 = 0;
    for n in 1..=num {
        sum = sum.wrapping_add(n);
        if sum == num {
            return 1;
        }
    }
    0
}
