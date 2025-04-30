fn sum_matrix_element_element_integer_division_row_column_1(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut temp: i32 = 0;
    let mut num: i32;
    let mut i: i32 = 1;
    while i <= n && temp < n {
        temp = i.wrapping_sub(1);
        num = 1;
        while temp < n {
            if temp.wrapping_add(i) <= n {
                ans = ans.wrapping_add(i.wrapping_mul(num));
            } else {
                ans = ans.wrapping_add((n.wrapping_sub(temp)).wrapping_mul(num));
            }
            temp = temp.wrapping_add(i);
            num = num.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    ans
}