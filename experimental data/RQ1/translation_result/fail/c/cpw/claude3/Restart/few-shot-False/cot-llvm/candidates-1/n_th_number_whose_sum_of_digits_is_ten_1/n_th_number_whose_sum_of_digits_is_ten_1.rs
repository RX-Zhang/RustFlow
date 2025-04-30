
fn n_th_number_whose_sum_of_digits_is_ten_1(n: i32) -> i32 {
    let mut count: i32 = 0;
    let mut curr = 19;

    loop {
        let mut sum: i32 = 0;
        let mut x = curr;

        while x > 0 {
            sum = sum.wrapping_add((x % 10) as i32);
            x = x / 10;
        }

        if sum == 10 {
            count = count.wrapping_add(1);
        }

        if count == n {
            return curr;
        }

        curr = curr.wrapping_add(9);
    }
}
