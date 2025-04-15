

fn check_number_is_perfect_square_using_addition_subtraction(n: i32) -> bool {
    let mut sum = 0;
    let mut i = 1;

    while sum < n {
        sum = sum.wrapping_add(i);
        if sum == n {
            return true;
        }
        i = i.wrapping_add(1).wrapping_add(1);
    }

    false
}
