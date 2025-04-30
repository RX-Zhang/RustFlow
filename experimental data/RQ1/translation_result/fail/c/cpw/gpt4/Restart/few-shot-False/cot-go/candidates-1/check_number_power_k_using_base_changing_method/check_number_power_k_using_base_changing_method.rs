
fn check_number_power_k_using_base_changing_method(n: u32, k: u32) -> u32 {
    let mut one_seen = false;
    let mut n = n;

    while n > 0 {
        let digit = n % k;
        if digit > 1 {
            return 0;
        }
        if digit == 1 {
            if one_seen {
                return 0;
            }
            one_seen = true;
        }
        n /= k;
    }
    return 1;
}
