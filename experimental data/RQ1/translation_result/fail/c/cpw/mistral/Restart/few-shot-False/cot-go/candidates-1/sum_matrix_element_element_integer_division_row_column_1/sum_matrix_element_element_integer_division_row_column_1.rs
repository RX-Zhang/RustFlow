
fn sum_matrix_element_element_integer_division_row_column_1(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut temp: i32;
    let mut num: i32;

    for i in 1..=n {
        temp = i - 1;
        num = 1;
        while temp < n {
            if temp + i <= n {
                ans = ans.wrapping_add(i.wrapping_mul(num));
            } else {
                ans = ans.wrapping_add((n - temp).wrapping_mul(num));
            }
            temp = temp.wrapping_add(i);
            num = num.wrapping_add(1);
        }
        if temp >= n {
            break;
        }
    }

    ans
}
