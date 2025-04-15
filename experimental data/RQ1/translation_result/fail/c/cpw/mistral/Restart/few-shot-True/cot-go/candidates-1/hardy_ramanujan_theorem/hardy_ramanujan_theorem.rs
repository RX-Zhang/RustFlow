
use std::iter::successors;

fn hardy_ramanujan_theorem(n: i32) -> i32 {
    let mut count = 0;
    let mut nn = n;

    // Check for even factors
    while nn % 2 == 0 {
        count += 1;
        nn /= 2;
    }

    // Check for odd prime factors
    for i in successors(Some(3), |&n| Some(n + 2)).take_while(|&i| i * i <= nn) {
        let mut m = nn;
        while m % i == 0 {
            count += 1;
            m /= i;
        }
    }

    // Check for remaining prime factor
    if nn > 2 {
        count += 1;
    }

    count
}
