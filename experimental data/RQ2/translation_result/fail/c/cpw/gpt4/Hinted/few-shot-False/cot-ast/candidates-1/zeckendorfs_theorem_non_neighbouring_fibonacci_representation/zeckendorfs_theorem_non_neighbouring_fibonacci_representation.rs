
use std::num::Wrapping;

fn zeckendorfs_theorem_non_neighbouring_fibonacci_representation(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    let mut f1 = Wrapping(0);
    let mut f2 = Wrapping(1);
    let mut f3 = Wrapping(1);
    while f3 <= Wrapping(n) {
        f1 = f2;
        f2 = f3;
        f3 = f1 + f2;
    }
    f2.0
}
