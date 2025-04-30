fn smallest_power_of_2_greater_than_or_equal_to_n_2(n: u32) -> u32 {
    let mut n_mut = n;
    n_mut = n_mut.wrapping_sub(1);
    n_mut |= n_mut.wrapping_shr(1);
    n_mut |= n_mut.wrapping_shr(2);
    n_mut |= n_mut.wrapping_shr(4);
    n_mut |= n_mut.wrapping_shr(8);
    n_mut |= n_mut.wrapping_shr(16);
    n_mut = n_mut.wrapping_add(1);
    n_mut
}