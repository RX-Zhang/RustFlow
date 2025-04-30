fn find_sum_odd_factors_number(n: i32) -> i32 {
    let mut res: i32 = 1;
    let mut n_mut = n;

    while n_mut.wrapping_rem(2) == 0 {
        n_mut = n_mut.wrapping_div(2);
    }

    let limit = (n_mut as f64).sqrt() as i32;
    let mut i: i32 = 3;
    while i <= limit {
        let mut count: i32 = 0;
        let mut curr_sum: i32 = 1;
        let mut curr_term: i32 = 1;

        while n_mut.wrapping_rem(i) == 0 {
            count = count.wrapping_add(1);
            n_mut = n_mut.wrapping_div(i);
            curr_term = curr_term.wrapping_mul(i);
            curr_sum = curr_sum.wrapping_add(curr_term);
        }

        res = res.wrapping_mul(curr_sum);
        i = i.wrapping_add(2);
    }

    if n_mut >= 2 {
        res = res.wrapping_mul(1_i32.wrapping_add(n_mut));
    }

    res
}