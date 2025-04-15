
fn count_possible_ways_to_construct_buildings(n: i32) -> i32 {
    if n == 1 {
        return 4;
    }
    let mut count_b: i64 = 1;
    let mut count_s: i64 = 1;
    let mut prev_count_b: i64;
    let mut prev_count_s: i64;
    for i in 2..=n {
        prev_count_b = count_b;
        prev_count_s = count_s;
        count_s = prev_count_b + prev_count_s;
        count_b = prev_count_s;
    }
    let result: i64 = count_s + count_b;
    return (result * result) as i32;
}
