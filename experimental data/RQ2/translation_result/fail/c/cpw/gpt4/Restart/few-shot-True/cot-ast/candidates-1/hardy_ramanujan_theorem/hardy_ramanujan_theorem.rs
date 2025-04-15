
use std::f64;

fn hardy_ramanujan_theorem(n: i32) -> i32 {
    let mut count = 0;
    let mut n = n;

    if n % 2 == 0 {
        count += 1;
        while n % 2 == 0 {
            n = n.wrapping_shr(1);
        }
    }

    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            count += 1;
            while n % i == 0 {
                n = n / i;
            }
        }
    }

    if n > 2 {
        count += 1;
    }

    count
}
