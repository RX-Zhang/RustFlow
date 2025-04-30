
fn n_th_number_whose_sum_of_digits_is_ten_1(n: i32) -> i32 {
    let mut count = 0;
    let mut curr = 19;

    loop {
        let mut sum = 0;
        let mut x = curr;

        while x > 0 {
            sum += x % 10;
            x /= 10;
        }

        if sum == 10 {
            count += 1;
        }

        if count == n {
            return curr;
        }

        curr = curr.wrapping_add(9);
    }
}
