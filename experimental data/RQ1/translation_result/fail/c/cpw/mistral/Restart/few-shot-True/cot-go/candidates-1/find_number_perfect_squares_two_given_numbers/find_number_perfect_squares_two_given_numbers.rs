

fn find_number_perfect_squares_two_given_numbers(a: i32, b: i32) -> i32 {
    let mut cnt = 0;
    for i in a..=b {
        for j in 1.. {
            let j_squared = (j as u32).wrapping_shl(1).wrapping_shl(1) as i32;
            if j_squared > i {
                break;
            } else if j_squared == i {
                cnt += 1;
                break;
            }
        }
    }
    cnt
}
