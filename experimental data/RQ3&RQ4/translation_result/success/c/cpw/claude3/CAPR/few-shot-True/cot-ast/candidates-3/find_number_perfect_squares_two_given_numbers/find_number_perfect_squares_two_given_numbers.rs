fn find_number_perfect_squares_two_given_numbers(a: i32, b: i32) -> i32 {
    let mut cnt: i32 = 0;

    let mut i: i32 = a;
    while i <= b {
        let mut j: i32 = 1;
        while j.wrapping_mul(j) <= i {
            if j.wrapping_mul(j) == i {
                cnt = cnt.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }

    cnt
}