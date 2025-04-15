

fn find_minimum_number_divided_make_number_perfect_square(n: i32) -> i32 {
    let mut count = 0;
    let mut ans = 1;
    let mut n = n;

    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }
    if count % 2 != 0 {
        ans *= 2;
    }
    let mut i = 3;
    while i * i <= n {
        count = 0;
        while n % i == 0 {
            count += 1;
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
