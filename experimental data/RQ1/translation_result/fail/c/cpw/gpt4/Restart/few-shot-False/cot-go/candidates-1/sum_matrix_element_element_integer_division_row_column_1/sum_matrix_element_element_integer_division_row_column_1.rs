
fn sum_matrix_element_element_integer_division_row_column_1(n: i32) -> i32 {
    let mut ans: i32 = 0; // Specify the type of `ans` as suggested
    let mut temp = 0;
    let mut num;
    
    for i in 1..=n {
        temp = i - 1;
        num = 1;
        
        while temp < n {
            // Use `wrapping_add` to prevent overflow, with explicit type for multiplication
            ans = if temp + i <= n {
                    ans.wrapping_add(i * num)
                } else {
                    ans.wrapping_add((n - temp) * num)
                };
            temp += i;
            num += 1;
        }
    }
    
    ans
}
