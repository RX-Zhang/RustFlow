

fn even_fibonacci_numbers_sum(limit: i32) -> i64 {
    if limit < 2 {
        return 0;
    }
    let mut ef1: i64 = 0;
    let mut ef2: i64 = 2;
    let mut sum: i64 = ef1 + ef2;
    while ef2 <= limit as i64 {
        let ef3 = (4 * ef2) as i64 + ef1;
        if ef3 > limit as i64 {
            break;
        }
        ef1 = ef2;
        ef2 = ef3;
        sum += ef2;
    }
    sum
}
