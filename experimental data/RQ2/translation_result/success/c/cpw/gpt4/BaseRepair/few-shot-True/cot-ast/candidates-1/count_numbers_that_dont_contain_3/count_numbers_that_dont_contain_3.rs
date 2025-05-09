fn count_numbers_that_dont_contain_3(n: i32) -> i32 {
    if n < 3 {
        return n;
    }
    if n >= 3 && n < 10 {
        return n - 1;
    }
    let mut po = 1;
    let mut temp = n;
    while temp / po > 9 {
        po = po.wrapping_mul(10);
    }
    let msd = n / po;
    if msd != 3 {
        count_numbers_that_dont_contain_3(msd)
            .wrapping_mul(count_numbers_that_dont_contain_3(po - 1))
            .wrapping_add(count_numbers_that_dont_contain_3(msd))
            .wrapping_add(count_numbers_that_dont_contain_3(n % po))
    } else {
        count_numbers_that_dont_contain_3(msd.wrapping_mul(po).wrapping_sub(1))
    }
}