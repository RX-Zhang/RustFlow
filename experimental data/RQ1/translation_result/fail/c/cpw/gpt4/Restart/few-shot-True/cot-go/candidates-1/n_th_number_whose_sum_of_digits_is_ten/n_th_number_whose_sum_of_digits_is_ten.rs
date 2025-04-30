
fn n_th_number_whose_sum_of_digits_is_ten(n: i32) -> i32 {
    let mut count = 0;
    let mut curr = 1;
    
    loop {
        let mut sum: i32 = 0;
        let mut x = curr;
        
        while x > 0 {
            sum = sum.wrapping_add(x % 10);
            x /= 10;
        }
        
        if sum == 10 {
            count += 1;
        }
        
        if count == n {
            return curr;
        }
        
        curr += 1;
    }
}
