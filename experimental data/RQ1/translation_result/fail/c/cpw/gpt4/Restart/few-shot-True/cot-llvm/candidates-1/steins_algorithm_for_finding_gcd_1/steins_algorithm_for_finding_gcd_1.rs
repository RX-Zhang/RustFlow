
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
    if a & 1 == 0 {
        if b & 1 != 0 {
            return steins_algorithm_for_finding_gcd_1(a.wrapping_shr(1), b);
        } else {
            return steins_algorithm_for_finding_gcd_1(a.wrapping_shr(1), b.wrapping_shr(1)).wrapping_shl(1);
        }
    }
    if b & 1 == 0 {
        return steins_algorithm_for_finding_gcd_1(a, b.wrapping_shr(1));
    }
    if a > b {
        return steins_algorithm_for_finding_gcd_1((a - b).wrapping_shr(1), b);
    }
    steins_algorithm_for_finding_gcd_1((b - a).wrapping_shr(1), a)
}
