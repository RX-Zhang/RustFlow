
fn check_number_is_perfect_square_using_additionsubtraction(n: i32) -> bool {
    let mut sum = 0;
    for i in (1..).step_by(2) {
        sum += i;
        if sum == n {
            return true;
        }
    }
    false
}
