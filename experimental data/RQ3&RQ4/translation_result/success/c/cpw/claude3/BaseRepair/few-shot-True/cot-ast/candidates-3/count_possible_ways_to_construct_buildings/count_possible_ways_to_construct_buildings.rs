fn count_possible_ways_to_construct_buildings(N: i32) -> i32 {
    if N == 1 {
        return 4;
    }
    let mut countB: i32= 1;
    let mut countS: i32 = 1;
    for _ in 2..=N {
        let prev_countB = countB;
        let prev_countS = countS;
        countS = prev_countB.wrapping_add(prev_countS);
        countB = prev_countS;
    }
    let result = countS.wrapping_add(countB);
    result.wrapping_mul(result)
}