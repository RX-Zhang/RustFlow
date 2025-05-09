

fn find_minimum_number_divided_make_number_perfect_square(n: i32) -> i32 {
    let mut count = 0;
    let mut ans = 1;
    let mut n = n;

    while n % 2 == 0 {
        count += 1;
        n = n.wrapping_shr(1);
    }

    if count % 2 == 1 {
        ans *= 2;
    }

    let mut i = 3;
    while i <= (n as f64).sqrt() as i32 {
        count = 0;
        n = n.wrapping_div(i as i32);

        while n % i == 0 {
            count += 1;
            n = n.wrapping_div(i as i32);
        }

        if count % 2 == 1 {
            ans *= i;
        }

        i += 2;
    

    if n > 2 {
        ans *= n;
    }

    ans
}
