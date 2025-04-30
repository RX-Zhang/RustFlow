
fn find_minimum_number_divided_make_number_perfect_square(mut n: i32) -> i32 {
    let mut count: i32 = 0;
    let mut ans = 1;
    
    while n % 2 == 0 {
        count = count.wrapping_add(1);
        n /= 2;
    }

    if count % 2 != 0 {
        ans *= 2;
    }

    let sqrt_n = (n as f64).sqrt() as i32;
    let mut i = 3;
    
    while i <= sqrt_n {
        count = 0;
        while n % i == 0 {
            count = count.wrapping_add(1);
            n /= i;
        }
        if count % 2 != 0 {
            ans *= i;
        }
        i += 2;
    }

    if n > 2 {
        ans *= n;
    }

    ans
}
