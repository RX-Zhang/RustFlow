
use std::cmp::Ordering;
use std::i32;

fn middle_of_three_using_minimum_comparisons_1(a: i32, b: i32, c: i32) -> i32 {
    if a.cmp(&b) == Ordering::Greater {
        if b.cmp(&c) == Ordering::Greater {
            b
        } else if a.cmp(&c) == Ordering::Greater {
            c
        } else {
            a
        }
    } else {
        if a.cmp(&c) == Ordering::Greater {
            a
        } else if b.cmp(&c) == Ordering::Greater {
            c
        } else {
            b
        }
    }
}
