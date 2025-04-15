
fn check_number_power_k_using_base_changing_method(n: u32, k: u32) -> i32 {
    let mut one_seen = false;
    let mut num = n;
    while num > 0 {
        let digit = num % k;
        if digit > 1 {
            return 0;
        }
        if digit == 1 {
            if one_seen {
                return 0;
            }
            one_seen = true;
        }
        num /= k;
    }
    1
}
