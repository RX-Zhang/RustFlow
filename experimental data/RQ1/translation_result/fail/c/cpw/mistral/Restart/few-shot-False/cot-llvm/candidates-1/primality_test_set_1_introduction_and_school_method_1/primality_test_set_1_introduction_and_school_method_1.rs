

fn primality_test_set_1_introduction_and_school_method_1(n: i32) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i.wrapping_add(6);
    }

    return true;
}
