
use std::uint32; // Remove this line as 'uint32' is not needed anymore

fn_power_of_(n: u32) -> bool
    if n == 0 {
        return false;
    }

    let mut count = 0;
    let mut num = n;
    while num > 1 {
        num = num.wrapping_shr(1);
        count += 1;
    }

    count % 2 == 0
}

