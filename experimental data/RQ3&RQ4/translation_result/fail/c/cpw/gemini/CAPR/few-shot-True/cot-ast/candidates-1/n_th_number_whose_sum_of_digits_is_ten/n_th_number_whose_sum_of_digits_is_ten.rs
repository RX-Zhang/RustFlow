
fn n_th_number_whose_sum_of_digits_is_ten(n: i32) -> i32 {
    let mut count = 0;
    for curr in 1.. {
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
    }
    -1
}
