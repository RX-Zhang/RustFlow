

fn hardy_ramanujan_theorem(n: i32) -> i32 {
    let mut count = 0;
    let mut nn = n;

    if nn % 2 == 0 {
        count += 1;
        nn = div_floor(&nn, 2);
    }

    let mut i = 3;
    while i <= (nn as f64).sqrt() as i32 {
        if nn % i == 0 {
            count += 1;
            nn = div_floor(&nn, i);
            while nn % i == 0 {
                nn = div_floor(&nn, i);
            }
        }
        i = i.wrapping_add(2);
    }

    if nn > 2 {
        count += 1;
    }

    count
}

fn div_floor(a: &i32, b: i32) -> i32 {
    (*a as f64 / b as f64).floor() as i32
}
