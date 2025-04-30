
fn fast_multiplication_method_without_using_multiplication_operator_russian_peasants_algorithm(a: u32, b: u32) -> u32 {
    let mut res: u32 = 0;
    let mut a = a;
    let mut b = b;
  
    while b > 0 {
        if b % 2 == 1 {
            res = res.wrapping_add(a);
        }
        a = a.wrapping_shl(1);
        b = b.wrapping_shr(1);
    }
    res
}
