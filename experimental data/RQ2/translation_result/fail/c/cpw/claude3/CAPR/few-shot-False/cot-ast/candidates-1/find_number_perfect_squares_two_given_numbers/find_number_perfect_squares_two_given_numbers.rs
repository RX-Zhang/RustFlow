
fn find_number_perfect_squares_two_given_numbers(a: i32, b: i32) -> i32 {
    let mut cnt: i32 = 0;
    for i in a..=b {
        for j in 1..=((i as f64).sqrt() as i32) {
            if j.wrapping_mul(j) == i {
                cnt = cnt.wrapping_add(1);
            }
        }
    }
    cnt
}
