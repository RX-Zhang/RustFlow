fn program_check_plus_perfect_number(x: i32) -> i32 {
    let temp = x;
    let mut n: i32 = 0;
    let mut x_mut = x;
    while x_mut != 0 {
        x_mut = x_mut.wrapping_div(10);
        n = n.wrapping_add(1);
    }
    x_mut = temp;
    let mut sum: i32 = 0;
    while x_mut != 0 {
        sum = sum.wrapping_add((x_mut % 10).pow(n as u32));
        x_mut = x_mut.wrapping_div(10);
    }
    if sum == temp { 1 } else { 0 }
}