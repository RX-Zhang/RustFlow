
fn middle_of_three_using_minimum_comparisons_1(a: i32, b: i32, c: i32) -> i32 {
    if a > b {
        if b > c {
            return b;
        } else if a > c {
            return c;
        } else {
            return a;
        }
    } else {
        if a > c {
            return a;
        } else if b > c {
            return c;
        } else {
            return b;
        }
    }
}
