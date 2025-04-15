
fn check_number_power_k_using_base_changing_method(n: u32, k: u32) -> bool {
    let mut n = n;
    let mut one_seen = false;

    while n > 0 {
        let digit = n % k;
        if digit > 1 {
            return false;
        }
        if digit == 1 {
            if one_seen {
                return false;
            }
            one_seen = true;
        }
        n /= k;
    }
    true
}
