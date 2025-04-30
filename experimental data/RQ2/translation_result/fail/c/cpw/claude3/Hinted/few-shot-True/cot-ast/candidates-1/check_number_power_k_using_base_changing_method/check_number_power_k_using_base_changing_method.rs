
fn check_number_power_k_using_base_changing_method(n: u32, k: u32) -> i32 {
    let mut n_mut = n;
    let mut one_seen = false;
    
    while n_mut > 0 {
        let digit = n_mut % k;
        if digit > 1 {
            return 0;
        }
        if digit == 1 {
            if one_seen {
                return 0;
            }
            one_seen = true;
        }
        n_mut = n_mut.wrapping_div(k);
    }
    1
}
