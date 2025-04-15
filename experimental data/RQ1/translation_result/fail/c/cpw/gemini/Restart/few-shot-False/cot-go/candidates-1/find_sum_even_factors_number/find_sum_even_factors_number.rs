
fn find_sum_even_factors_number(mut n: i32) -> i32 {
    if n % 2 != 0 {
        return 0;
    }
    let mut res: i64 = 1; // Use i64 to avoid overflow
    for i in 2..=(n as f64).sqrt() as i32 {
        let mut count = 0;
        let mut curr_sum: i64 = 1; // Use i64 to avoid overflow
        let mut curr_term: i64 = 1; // Use i64 to avoid overflow
        while n % i == 0 {
            count += 1;
            n /= i;
            if i == 2 && count == 1 {
                curr_sum = 0;
            }
            curr_term *= i as i64; // Cast to i64
            curr_sum += curr_term;
        }
        res *= curr_sum;
    }
    if n >= 2 {
        res *= (1 + n as i64); // Cast to i64
    }
    res as i32 // Cast back to i32 for the return type
}
