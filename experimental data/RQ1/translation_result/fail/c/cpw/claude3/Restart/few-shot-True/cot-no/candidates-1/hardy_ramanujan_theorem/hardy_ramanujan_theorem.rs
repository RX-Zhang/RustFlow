
fn hardy_ramanujan_theorem(mut n: i32) -> i32 {
    let mut count: i32 = 0;
    
    if n % 2 == 0 {
        count = count.wrapping_add(1);
        while n % 2 == 0 {
            n = n.wrapping_div(2);
        }
    }

    let sqrt_n = (n as f64).sqrt() as i32;
    
    let mut i = 3;
    while i <= sqrt_n {
        if n % i == 0 {
            count = count.wrapping_add(1);
            while n % i == 0 {
                n = n.wrapping_div(i);
            }
        }
        i = i.wrapping_add(2);
    }

    if n > 2 {
        count = count.wrapping_add(1);
    }

    count
}
