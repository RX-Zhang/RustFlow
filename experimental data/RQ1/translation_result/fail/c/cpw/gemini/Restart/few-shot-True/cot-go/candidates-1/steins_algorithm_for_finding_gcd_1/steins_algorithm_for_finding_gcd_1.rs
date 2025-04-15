
fn steins_algorithm_for_finding_gcd_1(a: i32, b: i32) -> i32 {
    if a == b {
        return a;
    }
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if (a & 1) == 0 {
        if (b & 1) != 0 {
            return steins_algorithm_for_finding_gcd_1(a >> 1, b);
        } else {
            return steins_algorithm_for_finding_gcd_1(a >> 1, b >> 1) << 1;
        }
    }
    if (b & 1) == 0 {
        return steins_algorithm_for_finding_gcd_1(a, b >> 1);
    }
    if a > b {
        return steins_algorithm_for_finding_gcd_1((a - b) >> 1, b);
    }
    return steins_algorithm_for_finding_gcd_1((b - a) >> 1, a);
}
