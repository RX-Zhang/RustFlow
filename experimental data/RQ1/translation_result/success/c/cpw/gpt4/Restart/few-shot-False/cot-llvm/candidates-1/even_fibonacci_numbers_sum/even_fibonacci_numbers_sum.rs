
pub fn even_fibonacci_numbers_sum(limit: i32) -> i32 {
    if limit < 2 {
        return 0;
    }
    let mut ef1: i32 = 0;
    let mut ef2: i32 = 2;
    let mut sum: i32 = ef1.wrapping_add(ef2);
    while ef2 <= limit {
        let ef3: i32 = 4 * ef2 + ef1;
        if ef3 > limit {
            break;
        }
        ef1 = ef2;
        ef2 = ef3;
        sum = sum.wrapping_add(ef2);
    }
    sum
}
