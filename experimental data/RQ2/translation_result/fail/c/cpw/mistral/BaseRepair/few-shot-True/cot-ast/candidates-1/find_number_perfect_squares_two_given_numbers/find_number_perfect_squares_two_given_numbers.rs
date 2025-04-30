

fn find_number_perfect_squares_two_given_numbers(a: i32, b: i32) -> i32 {
    let mut cnt = 0;
    for i in a..=b {
        for j in 1.. {
            let j_sq = j * j;
            if j_sq > i {
                break;
            } else if j_sq == i {
                cnt += 1;
                break;
            }
        }
    }
    cnt
}
