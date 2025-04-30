

fn hardy_ramanujan_theorem(n: i32) -> i32 {
    let mut count = 0;
    let mut num = n;
    if num % 2 == 0 {
        count += 1;
        while num % 2 == 0 {
            num /= 2;
        }
    }
    let limit = (num as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        if num % i == 0 {
            count += 1;
            while num % i == 0 {
                num /= i;
            }
        }
    }
    if num > 2 {
        count += 1;
    }
    count
}
