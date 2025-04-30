
fn hardy_ramanujan_theorem(mut n: i32) -> i32 {
    let mut count = 0;
    if n % 2 == 0 {
        count += 1;
        while n % 2 == 0 {
            n /= 2;
        }
    }
    for i in (3..=(n as f64).sqrt() as i32).step_by(2) {
        if n % i == 0 {
            count += 1;
            while n % i == 0 {
                n /= i;
            }
        }
    }
    if n > 2 {
        count += 1;
    }
    count
}
