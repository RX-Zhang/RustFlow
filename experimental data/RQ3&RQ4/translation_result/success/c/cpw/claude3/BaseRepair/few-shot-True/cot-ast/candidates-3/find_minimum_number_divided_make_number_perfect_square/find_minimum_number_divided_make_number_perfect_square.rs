fn find_minimum_number_divided_make_number_perfect_square(n: i32) -> i32 {
    let mut n = n;
    let mut count;
    let mut ans: i32 = 1;

    // Handle factor of 2
    count = 0;
    while n.wrapping_rem(2) == 0 {
        count += 1;
        n = n.wrapping_div(2);
    }
    if count % 2 != 0 {
        ans = ans.wrapping_mul(2);
    }

    // Handle odd factors
    let mut i: i32 = 3;
    while i <= (n as f64).sqrt() as i32 {
        count = 0;
        while n.wrapping_rem(i) == 0 {
            count += 1;
            n = n.wrapping_div(i);
        }
        if count % 2 != 0 {
            ans = ans.wrapping_mul(i);
        }
        i = i.wrapping_add(2);
    }

    // If remaining n is greater than 2
    if n > 2 {
        ans = ans.wrapping_mul(n);
    }

    ans
}