
fn find_number_perfect_squares_two_given_numbers(a: i32, b: i32) -> i32 {
    let mut cnt = 0;
    for i in a..=b {
        for j in 1..=(i as f64).sqrt() as i32 {
            if j * j == i {
                cnt += 1;
            }
        }
    }
    cnt
}
