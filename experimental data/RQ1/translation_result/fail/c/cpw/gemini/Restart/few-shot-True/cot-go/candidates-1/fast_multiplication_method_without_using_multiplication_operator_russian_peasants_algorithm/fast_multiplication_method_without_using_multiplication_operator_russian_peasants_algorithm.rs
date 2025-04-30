
fn fast_multiplication_method_without_using_multiplication_operator_russian_peasants_algorithm(mut a: u32, mut b: u32) -> u32 {
    let mut res: u32 = 0;
    while b > 0 {
        if b % 2 == 1 {
            res = res.wrapping_add(a);
        }
        a = a << 1;
        b = b >> 1;
    }
    res
}
