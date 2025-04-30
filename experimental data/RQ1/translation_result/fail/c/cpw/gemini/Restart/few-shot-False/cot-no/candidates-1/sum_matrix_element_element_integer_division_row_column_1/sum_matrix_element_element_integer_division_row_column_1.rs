
fn sum_matrix_element_element_integer_division_row_column_1(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut temp: i32 = 0;
    for i in 1..=n {
        if temp >= n {
            break;
        }
        temp = i - 1;
        let mut num: i32 = 1;
        while temp < n {
            if temp + i <= n {
                ans = ans.wrapping_add(i * num);
            } else {
                ans = ans.wrapping_add((n - temp) * num);
            }
            temp = temp.wrapping_add(i);
            num += 1;
        }
    }
    ans
}
