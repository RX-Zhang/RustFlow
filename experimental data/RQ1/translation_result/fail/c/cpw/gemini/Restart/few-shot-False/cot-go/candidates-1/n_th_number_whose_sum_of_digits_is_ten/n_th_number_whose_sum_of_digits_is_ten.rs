
fn n_th_number_whose_sum_of_digits_is_ten(n: i32) -> i32 {
    let mut count: i32 = 0;
    let mut curr: i32 = 1;
    loop {
        let mut sum: i32 = 0;
        let mut x: i32 = curr;
        while x > 0 {
            sum = sum.wrapping_add(x % 10);
            x = x / 10;
        }
        if sum == 10 {
            count = count.wrapping_add(1);
        }
        if count == n {
            return curr;
        }
        curr = curr.wrapping_add(1);
    }
}
