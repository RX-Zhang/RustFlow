
fn sum_matrix_element_element_integer_division_row_column(n: i32) -> i32 {
    let mut ans: i32 = 0;
    for i in 1..=n {
        for j in 1..=n {
            ans = ans.wrapping_add(i.wrapping_div(j)); 
        }
    }
    ans
}
