
fn sum_matrix_element_element_integer_division_row_column_1(n: i32) -> i32 {
    let mut ans = 0;
    let mut temp = 0;
    let mut num: i32;

    for i in 1..=n {
        if temp >= n {
            break;
        }
        temp = i - 1;
        num = 1;
        while temp < n {
            if temp + i <= n {
                ans += i * num;
            } else {
                ans += (n - temp) * num;
            }
            temp += i;
            num += 1;
        }
    }

    ans
}
