
use std::i32;

fn count_numbers_that_dont_contain_3(n: i32) -> i32 {
    if n < 3 {
        return n;
    }
    if n >= 3 && n < 10 {
        return n - 1;
    }
    let po = 1;
    let mut po = 1;
    while (n / po) > 9 {
        po *= 10;
    }
    let msd = n / po;
    if msd != 3 {
        return (count_numbers_that_dont_contain_3(msd) * count_numbers_that_dont_contain_3(po - 1)) + count_numbers_that_dont_contain_3(msd) + count_numbers_that_dont_contain_3(n % po);
    } else {
        return count_numbers_that_dont_contain_3(msd * po - 1);
    }
}
