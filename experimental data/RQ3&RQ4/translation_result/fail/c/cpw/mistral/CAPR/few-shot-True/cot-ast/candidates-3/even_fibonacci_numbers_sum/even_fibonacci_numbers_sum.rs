
use std::i64;
use std::num::Wrapping;

fn even_fibonacci_numbers_sum(limit: i64) -> i64 {
    if limit < 2 {
        return 0;
    }
    let mut ef1 = Wrapping(0);
    let mut ef2 = Wrapping(2);
    let mut sum = ef1.0 + ef2.0;
    while ef2.0 <= limit {
        let ef3 = ((ef2.0 << 2) + ef1.0) & i64::MAX as i64;
        if ef3 > limit {
            break;
        }
        ef1 = ef2;
        ef2 = Wrapping(ef3);
        sum += ef2.0;
    }
    sum
}
