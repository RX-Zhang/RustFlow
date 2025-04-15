
fn k_th_digit_raised_power_b(a: i32, b: i32, k: i32) -> i32 {
    let mut p: i64 = 1;

    if b < 0 {
        return if a == -1 {
            1
        } else {
            0
        }
    }

    let mut base = a;
    let mut exp = b as u32;
    let mut is_negative = false;

    if base < 0 {
        is_negative = exp % 2 == 1;
        base = base.abs();
    }

    while exp > 0 {
        if exp % 2 == 1 {
            p = p.wrapping_mul(base as i64);
        }
        base = base.wrapping_mul(base) as i32;
        exp /= 2;
    }

    if is_negative {
        p = -p;
    }

    let mut count = 0;
    let mut num = p as i32;

    while num != 0 && count < k {
        let rem = num % 10;
        count += 1;
        if count == k {
            return rem;
        }
        num /= 10;
    }

    0
}
