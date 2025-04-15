
fn count_possible_ways_to_construct_buildings(n: i32) -> i32 {
    if n == 1 {
        return 4;
    }
    let mut count_b = 1;
    let mut count_s = 1;
    for i in 2..=n {
        let prev_count_b = count_b;
        let prev_count_s = count_s;
        count_s = prev_count_b + prev_count_s;
        count_b = prev_count_s;
    }
    let result = count_s + count_b;
    return result * result;
}
