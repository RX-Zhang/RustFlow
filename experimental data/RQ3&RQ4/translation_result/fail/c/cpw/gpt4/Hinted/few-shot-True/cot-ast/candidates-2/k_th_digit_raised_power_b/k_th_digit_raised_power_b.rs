
use std::boxed::Box;

fn kth_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    if k <= 0 {
        return 0;
    }

    // Result may exceed the original i64, use wrapping_pow to handle large powers.
    let p = (a as i64).wrapping_pow(b as u32);

    let mut count = 0;
    let mut p = Box::new(p);

    while *p != 0 && count < k {
        let rem = *p % 10;
        count = count.wrapping_add(1);

        if count == k {
            return rem as i32;
        }
        
        *p = *p / 10;
    }

    0
}
