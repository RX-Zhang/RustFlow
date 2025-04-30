
fn find_number_perfect_squares_two_given_numbers(a: i32, b: i32) -> i32 {
    let mut cnt = 0;
    for i in a..=b {
        let mut j = 1;
        while j * j <= i {
            if j * j == i {
                cnt += 1;
            }
            j += 1;
        }
    }
    cnt
}
